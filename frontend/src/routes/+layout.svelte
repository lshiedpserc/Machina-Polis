<script>
	import '../app.css';
	let { children } = $props();
</script>

<div class="bg-background text-on-background font-body min-h-screen selection:bg-primary-container selection:text-on-primary-container">
	<!-- TopAppBar -->
	<header class="fixed top-0 w-full flex justify-between items-center px-8 h-16 bg-slate-950/80 backdrop-blur-xl z-50 shadow-[0_4px_30px_rgba(0,0,0,0.1)]">
		<div class="flex items-center gap-8">
			<span class="text-2xl font-bold bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent font-['Space_Grotesk'] tracking-tight">Machina Polis</span>
			<nav class="hidden md:flex items-center gap-6">
				<a class="text-slate-400 hover:text-blue-300 transition-colors" href="/">Dashboard</a>
				<a class="text-slate-400 hover:text-blue-300 transition-colors" href="/config">Configuration</a>
				<a class="text-slate-400 hover:text-blue-300 transition-colors" href="/stream">Chat Stream</a>
				<a class="text-slate-400 hover:text-blue-300 transition-colors" href="/history">History</a>
			</nav>
		</div>
		<div class="flex items-center gap-4">
			<button class="text-slate-400 hover:text-blue-300 scale-95 active:scale-90 transition-transform">
				<span class="material-symbols-outlined">notifications</span>
			</button>
			<button class="text-slate-400 hover:text-blue-300 scale-95 active:scale-90 transition-transform">
				<span class="material-symbols-outlined">settings</span>
			</button>
			<div class="w-8 h-8 rounded-full overflow-hidden border border-outline-variant">
				<!-- Mock avatar -->
				<div class="w-full h-full bg-blue-500"></div>
			</div>
		</div>
	</header>

	<!-- SideNavBar -->
	<aside class="fixed left-0 top-0 h-full w-64 bg-slate-950 border-r border-slate-800/50 flex flex-col py-6 z-40 hidden lg:flex pt-20">
		<div class="px-6 mb-8 mt-4">
			<div class="flex items-center gap-3 mb-6">
				<div class="w-10 h-10 rounded-xl bg-gradient-to-br from-blue-500 to-purple-600 flex items-center justify-center shadow-lg shadow-blue-500/20">
					<span class="material-symbols-outlined text-white" style="font-variation-settings: 'FILL' 1;">cloud_done</span>
				</div>
				<div>
					<h2 class="font-['Space_Grotesk'] font-bold text-white text-lg">AI Curator</h2>
					<p class="text-[10px] text-slate-500 uppercase tracking-widest">Active Session: Multi-Agent</p>
				</div>
			</div>
			<a href="/config" class="w-full py-3 px-4 bg-primary text-on-primary font-bold rounded-xl flex items-center justify-center gap-2 scale-95 hover:scale-100 transition-transform text-sm">
				<span class="material-symbols-outlined text-sm">add</span>
				<span>New Session</span>
			</a>
		</div>
		<nav class="flex-1 space-y-1">
			<a class="text-slate-500 py-3 px-6 flex items-center gap-4 hover:bg-slate-900 hover:text-slate-200 transition-all duration-300 ease-in-out font-['Inter'] text-sm" href="/">
				<span class="material-symbols-outlined">dashboard</span>
				<span>Dashboard</span>
			</a>
			<a class="text-slate-500 py-3 px-6 flex items-center gap-4 hover:bg-slate-900 hover:text-slate-200 transition-all duration-300 ease-in-out font-['Inter'] text-sm" href="/config">
				<span class="material-symbols-outlined">tune</span>
				<span>Configuration</span>
			</a>
			<a class="text-slate-500 py-3 px-6 flex items-center gap-4 hover:bg-slate-900 hover:text-slate-200 transition-all duration-300 ease-in-out font-['Inter'] text-sm" href="/stream">
				<span class="material-symbols-outlined">forum</span>
				<span>Chat Stream</span>
			</a>
			<a class="text-slate-500 py-3 px-6 flex items-center gap-4 hover:bg-slate-900 hover:text-slate-200 transition-all duration-300 ease-in-out font-['Inter'] text-sm" href="/history">
				<span class="material-symbols-outlined" style="font-variation-settings: 'FILL' 1;">history</span>
				<span>History</span>
			</a>
		</nav>
		<div class="px-6 mt-auto space-y-1 pt-6 border-t border-slate-900">
			<a class="text-slate-500 py-3 flex items-center gap-4 hover:text-slate-200 transition-all font-['Inter'] text-sm" href="/settings">
				<span class="material-symbols-outlined">settings</span>
				<span>Settings</span>
			</a>
			<a class="text-slate-500 py-3 flex items-center gap-4 hover:text-slate-200 transition-all font-['Inter'] text-sm" href="/support">
				<span class="material-symbols-outlined">help</span>
				<span>Support</span>
			</a>
		</div>
	</aside>

	<!-- Main Canvas -->
	<main class="lg:ml-64 pt-16 min-h-screen">
		{@render children()}
	</main>

	<!-- Mobile Navigation Bar -->
	<nav class="lg:hidden fixed bottom-0 left-0 w-full bg-slate-950/90 backdrop-blur-lg border-t border-slate-800/50 flex justify-around items-center h-16 z-50">
		<a href="/" class="flex flex-col items-center gap-1 text-slate-500">
			<span class="material-symbols-outlined">dashboard</span>
			<span class="text-[10px]">Dashboard</span>
		</a>
		<a href="/config" class="flex flex-col items-center gap-1 text-slate-500">
			<span class="material-symbols-outlined">tune</span>
			<span class="text-[10px]">Config</span>
		</a>
		<a href="/stream" class="flex flex-col items-center gap-1 text-slate-500">
			<span class="material-symbols-outlined">forum</span>
			<span class="text-[10px]">Stream</span>
		</a>
		<a href="/history" class="flex flex-col items-center gap-1 text-slate-500">
			<span class="material-symbols-outlined">history</span>
			<span class="text-[10px]">History</span>
		</a>
	</nav>
</div>
