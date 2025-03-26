<script lang="ts">
	import { onMount, onDestroy } from 'svelte';

	// Game Configuration Constants
	const GAME_CONFIG = {
		GRID_WIDTH: 10,
		GRID_HEIGHT: 20,
		CELL_SIZE: 30,
		GAME_SPEED: 500,
		COLORS: {
			BACKGROUND: '#f9f9f9',
			GRID_LINES: '#e0e0e0',
			PIECE_COLORS: [
				'#3498db', // I piece - blue
				'#e74c3c', // O piece - red
				'#2ecc71', // T piece - green
				'#f39c12', // L piece - orange
				'#9b59b6', // J piece - purple
				'#1abc9c', // S piece - teal
				'#e67e22'  // Z piece - dark orange
			],
			GRID_BORDER: '#2c3e50'
		}
	} as const;

	// Tetromino piece shapes
	const PIECES = [
		// I piece
		[[1,1,1,1]],
		
		// O piece
		[[1,1],[1,1]],
		
		// T piece
		[[0,1,0],[1,1,1]],
		
		// L piece
		[[1,0,0],[1,1,1]],
		
		// J piece
		[[0,0,1],[1,1,1]],
		
		// S piece
		[[0,1,1],[1,1,0]],
		
		// Z piece
		[[1,1,0],[0,1,1]]
	];

	type Position = { x: number; y: number };
	type GameStateType = 'running' | 'paused' | 'gameOver';

	class TetrisGame {
		public grid: number[][] = [];
		public currentPiece: number[][] = [];
		public currentPosition: Position = { x: 0, y: 0 };
		public score = 0;
		public state: GameStateType = 'running';
		private pieceColor = 0;

		initialize(): void {
			// Initialize empty grid
			this.grid = Array.from({ length: GAME_CONFIG.GRID_HEIGHT }, 
				() => Array(GAME_CONFIG.GRID_WIDTH).fill(0));
			
			this.spawnNewPiece();
			this.score = 0;
			this.state = 'running';
		}

		spawnNewPiece(): void {
			// Select random piece
			this.currentPiece = JSON.parse(JSON.stringify(
				PIECES[Math.floor(Math.random() * PIECES.length)]
			));
			
			// Select random color
			this.pieceColor = Math.floor(Math.random() * GAME_CONFIG.COLORS.PIECE_COLORS.length);
			
			// Start piece at top center
			this.currentPosition = { 
				x: Math.floor((GAME_CONFIG.GRID_WIDTH - this.currentPiece[0].length) / 2), 
				y: 0 
			};

			// Check for game over
			if (this.checkCollision()) {
				this.state = 'gameOver';
			}
		}

		move(dx: number, dy: number = 1): boolean {
			this.currentPosition.x += dx;
			this.currentPosition.y += dy;

			if (this.checkCollision()) {
				// Revert position
				this.currentPosition.x -= dx;
				this.currentPosition.y -= dy;

				// If moving down, lock piece in place
				if (dy > 0) {
					this.lockPiece();
					this.clearLines();
					this.spawnNewPiece();
				}
				return false;
			}
			return true;
		}

		rotatePiece(): void {
			// Rotate piece 90 degrees clockwise
			const rotated = this.currentPiece[0].map((_, index) => 
				this.currentPiece.map(row => row[index]).reverse()
			);

			const originalPiece = this.currentPiece;
			this.currentPiece = rotated;

			// Check if rotation is valid
			if (this.checkCollision()) {
				// Revert rotation if not valid
				this.currentPiece = originalPiece;
			}
		}

		private checkCollision(): boolean {
			for (let y = 0; y < this.currentPiece.length; y++) {
				for (let x = 0; x < this.currentPiece[y].length; x++) {
					if (this.currentPiece[y][x]) {
						const gridX = this.currentPosition.x + x;
						const gridY = this.currentPosition.y + y;

						if (
							gridX < 0 || 
							gridX >= GAME_CONFIG.GRID_WIDTH || 
							gridY >= GAME_CONFIG.GRID_HEIGHT ||
							(gridY >= 0 && this.grid[gridY][gridX])
						) {
							return true;
						}
					}
				}
			}
			return false;
		}

		private lockPiece(): void {
			for (let y = 0; y < this.currentPiece.length; y++) {
				for (let x = 0; x < this.currentPiece[y].length; x++) {
					if (this.currentPiece[y][x]) {
						const gridX = this.currentPosition.x + x;
						const gridY = this.currentPosition.y + y;
						
						if (gridY >= 0) {
							this.grid[gridY][gridX] = this.pieceColor + 1;
						}
					}
				}
			}
		}

		private clearLines(): void {
			let linesCleared = 0;
			for (let y = this.grid.length - 1; y >= 0; y--) {
				if (this.grid[y].every(cell => cell !== 0)) {
					// Remove line and add empty line at top
					this.grid.splice(y, 1);
					this.grid.unshift(Array(GAME_CONFIG.GRID_WIDTH).fill(0));
					y++; // Recheck same row
					linesCleared++;
				}
			}

			// Score based on lines cleared
			switch(linesCleared) {
				case 1: this.score += 100; break;
				case 2: this.score += 300; break;
				case 3: this.score += 500; break;
				case 4: this.score += 800; break;
			}
		}

		render(ctx: CanvasRenderingContext2D): void {
			if (!ctx) return;

			const { GRID_WIDTH, GRID_HEIGHT, CELL_SIZE } = GAME_CONFIG;
			const canvasWidth = GRID_WIDTH * CELL_SIZE;
			const canvasHeight = GRID_HEIGHT * CELL_SIZE;

			// Clear canvas
			ctx.clearRect(0, 0, canvasWidth, canvasHeight);

			// Draw background
			ctx.fillStyle = GAME_CONFIG.COLORS.BACKGROUND;
			ctx.fillRect(0, 0, canvasWidth, canvasHeight);

			// Draw grid lines
			ctx.strokeStyle = GAME_CONFIG.COLORS.GRID_LINES;
			for (let x = 0; x < GRID_WIDTH; x++) {
				for (let y = 0; y < GRID_HEIGHT; y++) {
					ctx.strokeRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
				}
			}

			// Draw locked pieces
			for (let y = 0; y < this.grid.length; y++) {
				for (let x = 0; x < this.grid[y].length; x++) {
					if (this.grid[y][x]) {
						ctx.fillStyle = GAME_CONFIG.COLORS.PIECE_COLORS[this.grid[y][x] - 1];
						ctx.fillRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
						ctx.strokeStyle = GAME_CONFIG.COLORS.GRID_BORDER;
						ctx.strokeRect(x * CELL_SIZE, y * CELL_SIZE, CELL_SIZE, CELL_SIZE);
					}
				}
			}

			// Draw current piece
			if (this.state !== 'gameOver') {
				ctx.fillStyle = GAME_CONFIG.COLORS.PIECE_COLORS[this.pieceColor];
				for (let y = 0; y < this.currentPiece.length; y++) {
					for (let x = 0; x < this.currentPiece[y].length; x++) {
						if (this.currentPiece[y][x]) {
							ctx.fillRect(
								(this.currentPosition.x + x) * CELL_SIZE, 
								(this.currentPosition.y + y) * CELL_SIZE, 
								CELL_SIZE, 
								CELL_SIZE
							);
							ctx.strokeStyle = GAME_CONFIG.COLORS.GRID_BORDER;
							ctx.strokeRect(
								(this.currentPosition.x + x) * CELL_SIZE, 
								(this.currentPosition.y + y) * CELL_SIZE, 
								CELL_SIZE, 
								CELL_SIZE
							);
						}
					}
				}
			}
		}

		getState(): GameStateType {
			return this.state;
		}

		getScore(): number {
			return this.score;
		}
	}

	// Component variables
	let game: TetrisGame;
	let canvas: HTMLCanvasElement;
	let gameLoop: ReturnType<typeof setInterval> | null = null;

	function initializeGame(): void {
		game = new TetrisGame();
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
				game.move(0); // Move piece down
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

		// Handle game state and controls
		switch(e.key) {
			case 'p':
			case 'Escape':
				togglePause();
				break;
			case 'ArrowUp':
				if (game.getState() === 'running') 
					game.rotatePiece();
				break;
			case 'ArrowDown':
				if (game.getState() === 'running') 
					game.move(0, 1);
				break;
			case 'ArrowLeft':
				if (game.getState() === 'running') 
					game.move(-1);
				break;
			case 'ArrowRight':
				if (game.getState() === 'running') 
					game.move(1);
				break;
			case 'Enter':
				if (game.getState() === 'gameOver') resetGame();
				break;
		}

		// Re-render after key event
		const ctx = canvas.getContext('2d');
		if (ctx) game.render(ctx);
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
		<h2 class="text-2xl font-bold text-gray-800">Tetris</h2>
		<div class="text-xl font-semibold text-green-600">
			Score: {game?.getScore() ?? 0}
		</div>
	</div>
	
	<div class="relative">
		<canvas 
			bind:this={canvas}
			width={GAME_CONFIG.GRID_WIDTH * GAME_CONFIG.CELL_SIZE} 
			height={GAME_CONFIG.GRID_HEIGHT * GAME_CONFIG.CELL_SIZE}
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
				<li>🏹 Arrow Left/Right: Move piece horizontally</li>
				<li>🔄 Arrow Up: Rotate piece</li>
				<li>⬇️ Arrow Down: Speed up piece descent</li>
				<li>⏸️ P or ESC: Pause/Unpause the game</li>
				<li>🔁 Enter: Restart game when game over</li>
			</ul>
			<p>🧩 Objective: Complete lines to score points!</p>
			<p>⚠️ Don't let pieces stack to the top!</p>
		</div>
	</div>
</div>