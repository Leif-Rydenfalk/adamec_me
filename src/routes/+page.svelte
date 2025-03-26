<script>
	import SnakeGame from '$lib/Snake.svelte';
  import TetrisGame from '$lib/Tetris.svelte';
	import { onMount } from 'svelte';
  import "../app.css"

	// Project data
	const projects = [
		{
			title: 'Snake Game',
			description: 'Classic snake game built with Svelte and Canvas API',
			technologies: ['Svelte', 'TypeScript', 'Canvas'],
			icon: '🐍',
			component: SnakeGame
		},
    {
			title: 'Tetris Game',
			description: 'Classic game built with Svelte and Canvas API',
			technologies: ['Svelte', 'TypeScript', 'Canvas'],
			icon: '🐍',
			component: TetrisGame
		},
		{
			title: 'Task Management System',
			description: 'Full-stack todo app with user authentication',
			technologies: ['Node.js', 'Express', 'MongoDB', 'JWT'],
			icon: '📋',
			githubLink: 'https://github.com/yourusername/task-manager'
		}
	];

	// Skills data
	const skills = [
		{ category: 'Frontend', items: ['React', 'Svelte', 'Vue', 'TypeScript'] },
		{ category: 'Backend', items: ['Node.js', 'Express', 'Python', 'Django'] },
		{ category: 'Database', items: ['MongoDB', 'PostgreSQL', 'Firebase'] },
		{ category: 'Tools', items: ['Git', 'Docker', 'AWS', 'Webpack'] }
	];

	// Contact information
	const contact = {
		email: 'your.email@example.com',
		linkedin: 'https://linkedin.com/in/yourusername',
		github: 'https://github.com/yourusername'
	};

	// Active project state
	let activeProject = null;

	function setActiveProject(project) {
		activeProject = project;
	}
</script>

<div class="container mx-auto px-4 py-8 max-w-4xl">
	<header class="text-center mb-12">
		<h1 class="text-4xl font-bold text-gray-800 mb-4">Leif</h1>
		<p class="text-xl text-gray-600">Fullstack Developer | Creative Problem Solver</p>
	</header>

	<section class="mb-12">
		<h2 class="text-3xl font-semibold text-gray-800 mb-6">Interactive Projects</h2>
		<div class="grid md:grid-cols-3 gap-6">
			{#each projects as project}
				<div 
					class="bg-white shadow-lg rounded-lg p-6 hover:shadow-xl transition-all cursor-pointer"
					on:click={() => setActiveProject(project)}
				>
					<div class="text-5xl mb-4">{project.icon}</div>
					<h3 class="text-xl font-semibold mb-2">{project.title}</h3>
					<p class="text-gray-600 mb-4">{project.description}</p>
					<div class="flex flex-wrap gap-2">
						{#each project.technologies as tech}
							<span class="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full">
								{tech}
							</span>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	</section>

	{#if activeProject && activeProject.component}
		<section class="mb-12 text-center">
			<h2 class="text-3xl font-semibold text-gray-800 mb-6">
				{activeProject.title} Demo
			</h2>
			<div class="flex justify-center">
				<svelte:component this={activeProject.component} />
			</div>
		</section>
	{/if}

	<section class="mb-12">
		<h2 class="text-3xl font-semibold text-gray-800 mb-6">Skills</h2>
		<div class="grid md:grid-cols-2 gap-6">
			{#each skills as skillGroup}
				<div class="bg-white shadow-lg rounded-lg p-6">
					<h3 class="text-xl font-semibold mb-4">{skillGroup.category}</h3>
					<div class="flex flex-wrap gap-2">
						{#each skillGroup.items as skill}
							<span class="bg-green-100 text-green-800 text-sm px-3 py-1 rounded-full">
								{skill}
							</span>
						{/each}
					</div>
				</div>
			{/each}
		</div>
	</section>

	<footer class="text-center">
		<h2 class="text-3xl font-semibold text-gray-800 mb-6">Get in Touch</h2>
		<div class="flex justify-center space-x-4">
			<a 
				href={`mailto:${contact.email}`} 
				class="bg-blue-500 text-white px-6 py-3 rounded-lg hover:bg-blue-600 transition-colors"
			>
				Email Me
			</a>
			<a 
				href={contact.linkedin} 
				target="_blank" 
				class="bg-blue-700 text-white px-6 py-3 rounded-lg hover:bg-blue-800 transition-colors"
			>
				LinkedIn
			</a>
			<a 
				href={contact.github} 
				target="_blank" 
				class="bg-gray-800 text-white px-6 py-3 rounded-lg hover:bg-gray-900 transition-colors"
			>
				GitHub
			</a>
		</div>
	</footer>
</div>