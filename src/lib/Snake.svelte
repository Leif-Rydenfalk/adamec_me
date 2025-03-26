<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	// Game Configuration Constants
	const GAME_CONFIG = {
		GRID_SIZE: 15,
		CELL_SIZE: 30,
		GAME_SPEED: 200,
		COLORS: {
			BACKGROUND: '#f9f9f9',
			GRID_LINES: '#e0e0e0',
			SNAKE_GRADIENT_START: '#2ecc71',
			SNAKE_GRADIENT_END: '#27ae60',
			FOOD: '#e74c3c'
		}
	} as const;

	// Types for better type safety
	type Position = { x: number; y: number };
	type GameStateType = 'running' | 'paused' | 'gameOver';

	class SnakeGame {
		public snake: Position[] = [];
		public food: Position = { x: 0, y: 0 };
		public direction: Position = { x: 1, y: 0 };
		public nextDirection: Position = { x: 1, y: 0 };
		public score = 0;
		public state: GameStateType = 'running';

		initialize(): void {
			// Start snake at the center of the grid
			this.snake = [{
				x: Math.floor(GAME_CONFIG.GRID_SIZE / 2), 
				y: Math.floor(GAME_CONFIG.GRID_SIZE / 2)
			}];
			
			this.food = this.generateFood();
			this.direction = { x: 1, y: 0 };
			this.nextDirection = { x: 1, y: 0 };
			this.score = 0;
			this.state = 'running';
		}

		generateFood(): Position {
			let newFood: Position;
			do {
				newFood = {
					x: Math.floor(Math.random() * GAME_CONFIG.GRID_SIZE),
					y: Math.floor(Math.random() * GAME_CONFIG.GRID_SIZE)
				};
			} while (this.snake.some(segment => 
				segment.x === newFood.x && segment.y === newFood.y
			));
			return newFood;
		}

		move(): void {
			// Validate and update direction
			this.updateDirection();

			// Calculate new snake head position with wrapping
			const newHead: Position = {
				x: (this.snake[0].x + this.direction.x + GAME_CONFIG.GRID_SIZE) % GAME_CONFIG.GRID_SIZE,
				y: (this.snake[0].y + this.direction.y + GAME_CONFIG.GRID_SIZE) % GAME_CONFIG.GRID_SIZE
			};

			// Check for self-collision
			if (this.checkSelfCollision(newHead)) {
				this.state = 'gameOver';
				return;
			}

			// Add new head
			this.snake.unshift(newHead);

			// Check for food consumption
			if (this.checkFoodConsumption(newHead)) {
				this.score++;
				this.food = this.generateFood();
			} else {
				// Remove tail if no food eaten
				this.snake.pop();
			}
		}

		private updateDirection(): void {
			// Prevent immediate reversal of direction
			const canChangeDirection = 
				(this.nextDirection.x !== -this.direction.x || this.snake.length === 1) && 
				(this.nextDirection.y !== -this.direction.y || this.snake.length === 1);

			if (canChangeDirection) {
				this.direction = this.nextDirection;
			}
		}

		private checkSelfCollision(head: Position): boolean {
			return this.snake.some(segment => 
				segment.x === head.x && segment.y === head.y
			);
		}

		private checkFoodConsumption(head: Position): boolean {
			return head.x === this.food.x && head.y === this.food.y;
		}

		render(ctx: CanvasRenderingContext2D): void {
			if (!ctx) return;

			const { GRID_SIZE, CELL_SIZE } = GAME_CONFIG;
			const canvasSize = GRID_SIZE * CELL_SIZE;

			// Clear canvas
			ctx.clearRect(0, 0, canvasSize, canvasSize);

			// Draw background and grid
			this.drawBackground(ctx);

			// Draw snake
			this.drawSnake(ctx);

			// Draw food
			this.drawFood(ctx);
		}

		private drawBackground(ctx: CanvasRenderingContext2D): void {
			const { GRID_SIZE, CELL_SIZE } = GAME_CONFIG;
			const canvasSize = GRID_SIZE * CELL_SIZE;

			// Fill background
			ctx.fillStyle = GAME_CONFIG.COLORS.BACKGROUND;
			ctx.fillRect(0, 0, canvasSize, canvasSize);
			
			// Draw grid lines
			ctx.strokeStyle = GAME_CONFIG.COLORS.GRID_LINES;
			for (let x = 0; x < GRID_SIZE; x++) {
				for (let y = 0; y < GRID_SIZE; y++) {
					ctx.strokeRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
				}
			}
		}

		private drawSnake(ctx: CanvasRenderingContext2D): void {
			const { CELL_SIZE } = GAME_CONFIG;
			const gradient = ctx.createLinearGradient(0, 0, CELL_SIZE * GAME_CONFIG.GRID_SIZE, CELL_SIZE * GAME_CONFIG.GRID_SIZE);
			gradient.addColorStop(0, GAME_CONFIG.COLORS.SNAKE_GRADIENT_START);
			gradient.addColorStop(1, GAME_CONFIG.COLORS.SNAKE_GRADIENT_END);
			ctx.fillStyle = gradient;

			this.snake.forEach((segment) => {
				ctx.beginPath();
				ctx.roundRect(
					segment.x * CELL_SIZE + 2, 
					segment.y * CELL_SIZE + 2, 
					CELL_SIZE - 4, 
					CELL_SIZE - 4, 
					5
				);
				ctx.fill();
			});
		}

		private drawFood(ctx: CanvasRenderingContext2D): void {
			const { CELL_SIZE } = GAME_CONFIG;
			ctx.fillStyle = GAME_CONFIG.COLORS.FOOD;
			const pulse = Math.sin(Date.now() * 0.01) * 3;
			
			ctx.beginPath();
			ctx.arc(
				this.food.x * CELL_SIZE + CELL_SIZE / 2, 
				this.food.y * CELL_SIZE + CELL_SIZE / 2, 
				CELL_SIZE / 2 - 5 + pulse, 
				0, 
				Math.PI * 2
			);
			ctx.fill();
		}

		// Getters for component to access game state
		getState(): GameStateType {
			return this.state;
		}

		getScore(): number {
			return this.score;
		}
	}

	// Component variables
	let game: SnakeGame;
	let canvas: HTMLCanvasElement;
	let gameLoop: ReturnType<typeof setInterval> | null = null;

	function initializeGame(): void {
		game = new SnakeGame();
		game.initialize();
		startGameLoop();
	}

	function startGameLoop(): void {
		// Clear any existing game loop
		if (gameLoop) {
			clearInterval(gameLoop);
		}

		gameLoop = setInterval(() => {
			if (game.getState() === 'running') {
				game.move();
				const ctx = canvas.getContext('2d');
				if (ctx) game.render(ctx);
			}
		}, GAME_CONFIG.GAME_SPEED);
	}

	function handleKeydown(e: KeyboardEvent): void {
		// Prevent default scrolling for arrow keys
		if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', 'p', 'Escape', 'Enter'].includes(e.key)) {
			e.preventDefault();
		}

		// Handle game state and direction controls
		switch(e.key) {
			case 'p':
			case 'Escape':
				togglePause();
				break;
			case 'ArrowUp':
				if (game.getState() === 'running') 
					game.nextDirection = { x: 0, y: -1 };
				break;
			case 'ArrowDown':
				if (game.getState() === 'running') 
					game.nextDirection = { x: 0, y: 1 };
				break;
			case 'ArrowLeft':
				if (game.getState() === 'running') 
					game.nextDirection = { x: -1, y: 0 };
				break;
			case 'ArrowRight':
				if (game.getState() === 'running') 
					game.nextDirection = { x: 1, y: 0 };
				break;
			case 'Enter':
				if (game.getState() === 'gameOver') resetGame();
				break;
		}
	}

	function togglePause(): void {
		if (game.getState() === 'gameOver') return;

		const currentState = game.getState();
		if (currentState === 'running') {
			game.state = 'paused';
			if (gameLoop) clearInterval(gameLoop);
		} else if (currentState === 'paused') {
			game.state = 'running';
			startGameLoop();
		}
	}

	function resetGame(): void {
		game.initialize();
		startGameLoop();
	}

	onMount(() => {
		// Initialize game and bind canvas
		initializeGame();
		
		// Add event listener
		window.addEventListener('keydown', handleKeydown);

		// Return cleanup function
		return () => {
			window.removeEventListener('keydown', handleKeydown);
			if (gameLoop) clearInterval(gameLoop);
		};
	});

	onDestroy(() => {
		if (gameLoop) clearInterval(gameLoop);
	});
</script>

<div class="flex flex-col items-center justify-center max-w-xl mx-auto bg-gray-100 p-6 rounded-xl shadow-lg relative">
	<div class="flex items-center justify-between w-full mb-4">
		<h2 class="text-2xl font-bold text-gray-800">Snake</h2>
		<div class="text-xl font-semibold text-green-600">
			Score: {game?.getScore() ?? 0}
		</div>
	</div>
	
	<div class="relative">
		<canvas 
			bind:this={canvas}
			width={GAME_CONFIG.GRID_SIZE * GAME_CONFIG.CELL_SIZE} 
			height={GAME_CONFIG.GRID_SIZE * GAME_CONFIG.CELL_SIZE}
			class="border-4 border-gray-800 rounded-lg shadow-md"
		></canvas>

		{#if game?.getState() === 'gameOver'}
			<div class="absolute inset-0 flex items-center justify-center bg-transparent">
				<div class="bg-white p-8 rounded-lg shadow-md text-center">
					<p class="text-3xl font-bold text-red-500 mb-4">Game Over!</p>
					<p class="text-xl text-gray-700 mb-2">Press Enter to Restart</p>
					<p class="text-2xl text-green-600 font-semibold">Final Score: {game?.getScore() ?? 0}</p>
				</div>
			</div>
		{/if}

		{#if game?.getState() === 'paused'}
			<div class="absolute inset-0 flex items-center justify-center bg-transparent">
				<div class="bg-white p-8 rounded-lg shadow-md text-center">
					<p class="text-3xl font-bold text-red-500 mb-4">Paused</p>
					<p class="text-xl text-gray-700">Press P or ESC to Unpause</p>
				</div>
			</div>
		{/if}
	</div>
	
	<div class="mt-6 text-center bg-white p-6 rounded-lg shadow-md">
		<h3 class="text-2xl font-bold text-gray-800 mb-4">How to Play</h3>
		<div class="text-left space-y-3 text-gray-700">
			<p>🎮 Controls:</p>
			<ul class="list-disc list-inside">
				<li>🏹 Arrow Keys: Move the snake</li>
				<li>⏸️ P or ESC: Pause/Unpause the game</li>
				<li>🔁 Enter: Restart game when game over</li>
			</ul>
			<p>🍎 Objective: Eat the red food to grow and increase your score!</p>
			<p>⚠️ Avoid running into yourself!</p>
		</div>
	</div>
</div>
