<svelte:head>
	<title>Leif Adamec Rydenfalk - Portfolio</title>
	<meta
		name="description"
		content="Portfolio of Leif Adamec Rydenfalk - Software Engineer, Graphics Specialist, Game Developer, and Creative Innovator"
	/>
</svelte:head>

<!-- Hero Section -->
<header
	class="relative flex min-h-screen items-center justify-center overflow-hidden bg-gradient-to-br from-blue-100 to-gray-200"
>
	<div class="absolute inset-0 opacity-20">
		<svg class="h-full w-full" xmlns="http://www.w3.org/2000/svg">
			<rect width="100%" height="100%" fill="url(#grid)" />
			<defs>
				<pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
					<path d="M 40 0 L 0 0 0 40" fill="none" stroke="gray" stroke-width="1" />
				</pattern>
			</defs>
		</svg>
	</div>
	<div class="z-10 container mx-auto px-6 text-center">
		<h1 class="mb-4 text-5xl font-bold text-gray-800 md:text-6xl">Leif Adamec Rydenfalk</h1>
		<p class="mb-6 text-xl text-blue-600 md:text-2xl">
			Software Engineer | Graphics Specialist | Game Developer
		</p>
		<p class="mb-8 text-lg text-gray-600 italic">
			"Architecting high-performance systems and immersive experiences at the intersection of code,
			graphics, and sound."
		</p>
		<div class="space-x-4">
			<a
				href="mailto:ledamecrydenfalk@gmail.com"
				class="inline-block rounded-lg bg-blue-500 px-6 py-3 text-white transition hover:bg-blue-600"
				>Contact Me</a
			>
			<!-- <a
				href="https://adamec.me"
				target="_blank"
				class="inline-block rounded-lg bg-gray-500 px-6 py-3 text-white transition hover:bg-gray-600"
				>Visit Website</a
			> -->
		</div>
	</div>
</header>

<!-- About Section -->
<section class="bg-white py-20">
	<div class="container mx-auto px-6">
		<h2 class="mb-12 text-center text-4xl font-bold text-gray-800">About Me</h2>
		<div class="mx-auto max-w-3xl space-y-6 text-lg leading-relaxed text-gray-600">
			<p>
				As a highly skilled Software Engineer with a deep focus on performance optimization and
				graphics programming, I specialize in creating efficient, high-performance systems and
				custom engines. My expertise spans advanced rendering techniques, multithreading, and
				physics simulations. With a strong foundation in languages like Rust, C++, and modern web
				technologies, I thrive in developing cutting-edge solutions for game engines, multiplayer
				systems, and interactive media.
			</p>
			<p>
				My journey began with a fascination for games and interactive media, leading me to master
				tools like Bevy (Rust), Unity, and Blender. I’ve contributed to open-source projects such as
				Godot, Rui (a SwiftUI-like GUI library for Rust), and a vector graphics rendering platform,
				while building personal projects like voxel renderers, real-time ray tracers, and custom
				physics engines. Beyond coding, I’m a creative innovator—composing music with Ableton and FL
				Studio and crafting 3D assets for immersive experiences.
			</p>
			<a
				href="/resume.pdf"
				download
				class="inline-block rounded-lg bg-blue-500 px-6 py-3 text-white transition hover:bg-blue-600"
				>Download Resume</a
			>
		</div>
	</div>
</section>

<!-- Skills Section -->
<section class="bg-gray-100 py-20">
	<div class="container mx-auto px-6">
		<h2 class="mb-12 text-center text-4xl font-bold text-gray-800">Technical Skills</h2>
		<div class="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3">
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Programming Languages</h3>
				<p class="text-gray-600">
					Rust (Expert), C, C++, Zig, JavaScript, TypeScript, SQL, Python, C#, HTML, CSS, Tailwind,
					HTMX
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Frontend Development</h3>
				<p class="text-gray-600">
					React.js, Vue.js, Angular, TypeScript, Redux, Vuex, Pinia, Tailwind CSS, Bootstrap,
					Material-UI, Responsive Design, MVC, SOLID Principles
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Backend Development</h3>
				<p class="text-gray-600">
					REST APIs, GraphQL, WebSockets, gRPC, Express.js, Django, Flask, Spring Boot, OAuth2, JWT,
					MongoDB, PostgreSQL, MySQL, ORM (TypeORM, Sequelize, Prisma)
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Graphics & Rendering</h3>
				<p class="text-gray-600">
					GLSL, WGSL, Vulkan, Ash, ImGui, WGPU, Vulkano, HLSL, Metal, Real-Time Ray Tracing, LOD
					Planet Rendering, Voxel Rendering, Procedural Content Generation
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Performance Optimization</h3>
				<p class="text-gray-600">
					SIMD, Multithreading, GPU Computing, Advanced Shading, Web Performance Optimization,
					Database Tuning, Caching (Redis, Memcached)
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Networking & Async Systems</h3>
				<p class="text-gray-600">
					Rust Async, Custom Networking, Multiplayer Design, WebRTC, Sockets, WebSockets
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Web Development</h3>
				<p class="text-gray-600">
					Svelte, SvelteKit, Rust (Dominator), WebAssembly, GraphQL, PWAs, Serverless, Microservices
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Game Development</h3>
				<p class="text-gray-600">
					Custom Engines, Bevy, Unity, Cross-Platform (PC, Console, Mobile, VR/AR), Multiplayer, AI,
					Animation, Physics
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Systems Programming</h3>
				<p class="text-gray-600">
					Distributed Systems, Debugging, Hardware Optimization, Docker, Kubernetes, CI/CD
				</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Cloud Platforms</h3>
				<p class="text-gray-600">AWS, Google Cloud, Azure, Firebase</p>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Audio & Creative Tools</h3>
				<p class="text-gray-600">
					Ableton, FL Studio, Adaptive Music, 3D Audio, Interactive Sound Programming
				</p>
			</div>
		</div>
	</div>
</section>

<!-- Experience Section -->
<section class="bg-white py-20">
	<div class="container mx-auto px-6">
		<h2 class="mb-12 text-center text-4xl font-bold text-gray-800">Professional Experience</h2>
		<div class="space-y-12">
			<div class="rounded-lg bg-gray-100 p-6 shadow-sm">
				<h3 class="text-2xl font-semibold text-blue-600">Freelance Software Engineer</h3>
				<p class="mb-4 text-gray-500">2022 - 2025</p>
				<ul class="list-inside list-disc space-y-2 text-gray-600">
					<li>
						Designed custom physics engines in Rust with rigidbody solvers and fluid dynamics.
					</li>
					<li>Developed optimized rendering engines for terrains, voxels, and LOD planets.</li>
					<li>Built high-performance multithreaded systems with SIMD optimizations.</li>
					<li>Created eCommerce platforms with SvelteKit and Firebase, improving load times.</li>
					<li>Developed a personal portfolio website using Svelte and SvelteKit.</li>
					<li>Engineered a Rust-based web app with Dominator for high performance.</li>
					<li>Integrated Firebase and SQL for robust database solutions.</li>
					<li>Implemented Rust async programming and socket-based networking.</li>
					<li>Built games and engines with Bevy and Unity, including multiplayer systems.</li>
					<li>Modeled and animated 3D assets in Blender for games and visualizations.</li>
					<li>Produced music with Ableton and FL Studio for personal and professional use.</li>
					<li>
						Developed Vulkan-based rendering engines with global illumination and path tracing.
					</li>
				</ul>
			</div>
			<div class="rounded-lg bg-gray-100 p-6 shadow-sm">
				<h3 class="text-2xl font-semibold text-blue-600">Open Source Contributor</h3>
				<p class="mb-4 text-gray-500">2022 - Present</p>
				<ul class="list-inside list-disc space-y-2 text-gray-600">
					<li>Enhanced Godot’s rendering pipeline with Vulkan and optimization.</li>
					<li>Contributed to Rui, a SwiftUI-like GUI library for Rust.</li>
					<li>Developed a vector graphics rendering platform in Rust.</li>
					<li>Participated in open-source game development projects.</li>
				</ul>
			</div>
		</div>
	</div>
</section>

<!-- Projects Section -->
<section class="bg-gray-100 py-20">
	<div class="container mx-auto px-6">
		<h2 class="mb-12 text-center text-4xl font-bold text-gray-800">Featured Projects</h2>
		<div class="grid grid-cols-1 gap-8 md:grid-cols-2 lg:grid-cols-3">
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Custom Game Engine</h3>
				<p class="mb-4 text-gray-600">
					High-performance engine in Rust with ECS, Vulkan rendering, and multiplayer support.
				</p>
				<a
					href="https://github.com/username/repo"
					target="_blank"
					class="text-blue-500 hover:underline">GitHub</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Bevy-Based Game</h3>
				<p class="mb-4 text-gray-600">
					A game built with Bevy, leveraging Rust’s performance for gameplay and networking.
				</p>
				<a
					href="https://github.com/username/repo"
					target="_blank"
					class="text-blue-500 hover:underline">GitHub</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Unity Game Development</h3>
				<p class="mb-4 text-gray-600">Designed gameplay mechanics and assets in Unity.</p>
				<a
					href="https://github.com/username/repo"
					target="_blank"
					class="text-blue-500 hover:underline">GitHub</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">LOD Planet Renderer</h3>
				<p class="mb-4 text-gray-600">
					Optimized renderer for space visualization with real-time LOD and procedural generation.
				</p>
				<a href="https://example.com/demo" target="_blank" class="text-blue-500 hover:underline"
					>Live Demo</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Voxel Renderer</h3>
				<p class="mb-4 text-gray-600">
					Scalable voxel system with multithreading and advanced shading.
				</p>
				<a
					href="https://github.com/username/repo"
					target="_blank"
					class="text-blue-500 hover:underline">GitHub</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Custom Physics Engine</h3>
				<p class="mb-4 text-gray-600">
					Rust-based engine with rigidbody solvers, soft bodies, and fluid dynamics.
				</p>
				<a
					href="https://github.com/username/repo"
					target="_blank"
					class="text-blue-500 hover:underline">GitHub</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Real-Time Ray Tracer</h3>
				<p class="mb-4 text-gray-600">
					Vulkan-based ray tracer with photorealistic rendering and global illumination.
				</p>
				<a
					href="https://github.com/username/repo"
					target="_blank"
					class="text-blue-500 hover:underline">GitHub</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">E-commerce Platform</h3>
				<p class="mb-4 text-gray-600">
					Modern eCommerce site with SvelteKit, Firebase, and performance optimizations.
				</p>
				<a href="https://example.com/demo" target="_blank" class="text-blue-500 hover:underline"
					>Live Demo</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Rust Web Application</h3>
				<p class="mb-4 text-gray-600">High-performance web app built with Rust and Dominator.</p>
				<a
					href="https://github.com/username/repo"
					target="_blank"
					class="text-blue-500 hover:underline">GitHub</a
				>
			</div>
			<div class="rounded-lg bg-white p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Personal Website</h3>
				<p class="mb-4 text-gray-600">Portfolio site built with Svelte and SvelteKit.</p>
				<a href="https://adamec.me" target="_blank" class="text-blue-500 hover:underline"
					>Live Site</a
				>
			</div>
		</div>
	</div>
</section>

<!-- Open Source Contributions Section -->
<section class="bg-white py-20">
	<div class="container mx-auto px-6">
		<h2 class="mb-12 text-center text-4xl font-bold text-gray-800">Open Source Contributions</h2>
		<div class="grid grid-cols-1 gap-8 md:grid-cols-2">
			<div class="rounded-lg bg-gray-100 p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Godot Engine</h3>
				<p class="text-gray-600">
					Improved rendering pipeline with Vulkan integration and performance enhancements.
				</p>
			</div>
			<div class="rounded-lg bg-gray-100 p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Rui (Rust GUI Library)</h3>
				<p class="text-gray-600">Added features to a SwiftUI-inspired GUI framework for Rust.</p>
			</div>
			<div class="rounded-lg bg-gray-100 p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Vector Graphics Platform</h3>
				<p class="text-gray-600">Developed a high-performance vector graphics renderer in Rust.</p>
			</div>
			<div class="rounded-lg bg-gray-100 p-6 shadow-sm">
				<h3 class="mb-4 text-xl font-semibold text-blue-600">Open Source Games</h3>
				<p class="text-gray-600">Contributed to various open-source game projects.</p>
			</div>
		</div>
	</div>
</section>

<!-- Music Section -->
<section class="bg-gray-100 py-8">
	<div class="container mx-auto space-y-8 px-6 text-center">
		<h2 class="mb-12 text-4xl font-bold text-gray-800">Music & Audio</h2>
		<p class="mb-6 text-lg text-gray-600">
			I compose and produce music with Ableton and FL Studio, specializing in adaptive soundtracks
			and 3D audio for games. Explore my work below!
		</p>
		<div class="space-y-4">
			<p class="text-gray-500 italic">Sample: "Old Man" - Game Soundtrack Demo</p>
			<audio controls class="mx-auto">
				<source src="/Old man.mp3" type="audio/mp3" />
				Your browser does not support the audio element.
			</audio>
		</div>
		<div class="space-y-4">
			<p class="text-gray-500 italic">Sample: "Morning Sun" - Game Soundtrack Demo</p>
			<audio controls class="mx-auto">
				<source src="/Morning sun.mp3" type="audio/mp3" />
				Your browser does not support the audio element.
			</audio>
		</div>
		<div class="space-y-4">
			<p class="text-gray-500 italic">Sample: "Medieval Warfare" - Game Soundtrack Demo</p>
			<audio controls class="mx-auto">
				<source src="/Medieval warfare.mp3" type="audio/mp3" />
				Your browser does not support the audio element.
			</audio>
		</div>
	</div>
</section>

<!-- Contact Section -->
<section class="bg-white py-20">
	<div class="container mx-auto px-6 text-center">
		<h2 class="mb-12 text-4xl font-bold text-gray-800">Get in Touch</h2>
		<p class="mb-6 text-lg text-gray-600">
			Interested in collaboration or discussing a project? Let’s connect!
		</p>
		<form class="mx-auto max-w-md space-y-4">
			<input
				type="text"
				placeholder="Name"
				class="w-full rounded-lg bg-gray-100 p-3 text-gray-800"
			/>
			<input
				type="email"
				placeholder="Email"
				class="w-full rounded-lg bg-gray-100 p-3 text-gray-800"
			/>
			<textarea
				placeholder="Message"
				class="w-full rounded-lg bg-gray-100 p-3 text-gray-800"
				rows="4"
			></textarea>
			<button
				type="submit"
				class="rounded-lg bg-blue-500 px-6 py-3 text-white transition hover:bg-blue-600"
				>Send</button
			>
		</form>
		<div class="mt-8 space-y-4">
			<p>
				Email: <a href="mailto:ledamecrydenfalk@gmail.com" class="text-blue-500 hover:underline"
					>ledamecrydenfalk@gmail.com</a
				>
			</p>
			<p>Phone: <span class="text-blue-500">+46 729 782 220</span></p>
			<p>
				Website: <a href="https://adamec.me" target="_blank" class="text-blue-500 hover:underline"
					>adamec.me</a
				>
			</p>
			<p>
				GitHub: <a
					href="https://github.com/username"
					target="_blank"
					class="text-blue-500 hover:underline">@username</a
				>
			</p>
		</div>
	</div>
</section>

<!-- Footer -->
<footer class="bg-gray-100 py-6 text-center text-gray-500">
	<p>© 2025 Leif Adamec Rydenfalk. All rights reserved.</p>
	<div class="mt-2">
		<a href="#top" class="text-blue-500 hover:underline">Back to Top</a>
	</div>
</footer>
