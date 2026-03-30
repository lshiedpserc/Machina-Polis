<script>
    import { goto } from '$app/navigation';

    let username = $state('');
    let password = $state('');

    /** @param {Event} e */
    function handleLogin(e) {
        e.preventDefault();
        // Mock login
        if (username) {
            goto('/');
        }
    }
</script>

<svelte:head>
    <title>Login | Machina Polis</title>
</svelte:head>

<!-- We use a fixed full-screen layout to cover the app layout elements -->
<div class="fixed inset-0 z-[100] bg-background flex flex-col items-center justify-center p-6">
    <!-- Background element -->
    <div class="absolute inset-0 z-0 overflow-hidden pointer-events-none">
        <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[800px] bg-primary/10 rounded-full blur-[100px]"></div>
    </div>

    <div class="relative z-10 w-full max-w-md">
        <div class="text-center mb-10">
            <h1 class="text-5xl font-bold bg-gradient-to-r from-blue-500 to-purple-500 bg-clip-text text-transparent font-['Space_Grotesk'] tracking-tight mb-4">Machina Polis</h1>
            <p class="text-on-surface-variant font-label">マルチAIセッションへようこそ</p>
        </div>

        <div class="bg-surface-container-low/80 backdrop-blur-xl p-8 rounded-3xl border border-outline-variant/20 shadow-2xl">
            <form onsubmit={handleLogin} class="space-y-6">
                <div>
                    <label for="username" class="block text-sm font-label font-bold text-on-surface mb-2">Username</label>
                    <div class="relative">
                        <span class="material-symbols-outlined absolute left-4 top-1/2 -translate-y-1/2 text-outline">person</span>
                        <input bind:value={username} type="text" id="username" class="w-full bg-surface-container-high border border-outline-variant rounded-xl py-3 pl-12 pr-4 text-on-surface placeholder:text-outline focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all" placeholder="Enter your username" required>
                    </div>
                </div>

                <div>
                    <div class="flex items-center justify-between mb-2">
                        <label for="password" class="block text-sm font-label font-bold text-on-surface">Password</label>
                        <a href="#" class="text-xs text-primary hover:text-primary-dim transition-colors">Forgot password?</a>
                    </div>
                    <div class="relative">
                        <span class="material-symbols-outlined absolute left-4 top-1/2 -translate-y-1/2 text-outline">lock</span>
                        <input bind:value={password} type="password" id="password" class="w-full bg-surface-container-high border border-outline-variant rounded-xl py-3 pl-12 pr-4 text-on-surface placeholder:text-outline focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all" placeholder="Enter your password" required>
                    </div>
                </div>

                <button type="submit" class="w-full py-4 bg-gradient-to-r from-primary to-secondary text-on-primary-container rounded-xl font-bold font-headline tracking-wide hover:shadow-[0_0_20px_rgba(129,236,255,0.3)] transition-all flex justify-center items-center gap-2 mt-4">
                    <span>ログイン</span>
                    <span class="material-symbols-outlined text-sm">login</span>
                </button>
            </form>

            <div class="mt-8 text-center text-sm text-on-surface-variant">
                アカウントをお持ちでないですか？ <a href="#" class="text-primary hover:underline font-bold">新規登録</a>
            </div>
        </div>
    </div>
</div>
