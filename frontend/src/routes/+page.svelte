<script>
    import { onMount } from 'svelte';
</script>

<svelte:head>
    <title>Machina Polis Dashboard</title>
</svelte:head>

<div class="p-8 lg:p-12 max-w-7xl mx-auto pb-24 lg:pb-12">
    <!-- Hero Section -->
    <section class="relative mb-16 rounded-3xl overflow-hidden min-h-[400px] flex items-center group">
        <div class="absolute inset-0 z-0">
            <div class="w-full h-full bg-gradient-to-r from-blue-900 to-purple-900 opacity-40 transition-transform duration-700 group-hover:scale-105"></div>
            <div class="absolute inset-0 bg-gradient-to-r from-background via-background/60 to-transparent"></div>
        </div>

        <div class="relative z-10 px-12 py-16 max-w-2xl">
            <h1 class="text-6xl md:text-7xl font-headline font-bold mb-6 tracking-tight leading-none text-white">Machina<br/><span class="text-primary">Polis</span></h1>
            <p class="text-lg text-on-surface-variant font-body mb-8 leading-relaxed">
                複数のAIエージェントが交差する、知性の新次元へ。データと直感が融合するマルチモーダル環境で、あなたの思考を拡張します。
            </p>
            <div class="flex flex-wrap gap-4">
                <a href="/config" class="px-8 py-4 bg-primary text-on-primary rounded-xl font-bold font-label tracking-wide hover:brightness-110 transition-all shadow-lg shadow-primary/20">
                    システム起動
                </a>
                <button class="px-8 py-4 border border-outline-variant/30 backdrop-blur-md rounded-xl font-bold font-label tracking-wide hover:bg-white/5 transition-all text-on-surface">
                    詳細を確認
                </button>
            </div>
        </div>

        <div class="absolute right-12 bottom-12 hidden lg:flex items-center gap-3">
            <div class="w-3 h-3 rounded-full bg-primary animate-pulse"></div>
            <span class="font-label text-xs uppercase tracking-[0.2em] text-on-surface-variant">Core Engine: V4.2 Online</span>
        </div>
    </section>

    <!-- Mode Selection: Bento Grid Style -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
        <!-- Observation Mode -->
        <div class="lg:col-span-7 flex flex-col">
            <div class="bg-surface-container-low/60 backdrop-blur-xl p-8 rounded-3xl border border-outline-variant/10 relative overflow-hidden flex-1 flex flex-col">
                <div class="absolute top-0 right-0 p-8">
                    <span class="material-symbols-outlined text-primary text-5xl opacity-20" data-icon="visibility">visibility</span>
                </div>

                <div class="relative z-10">
                    <span class="inline-block px-3 py-1 rounded-full bg-primary/10 text-primary text-[10px] font-bold tracking-widest uppercase mb-4">Discovery</span>
                    <h2 class="text-3xl font-headline font-bold text-white mb-4">観察モード</h2>
                    <p class="text-on-surface-variant mb-8 text-sm leading-relaxed max-w-md">
                        AIたちの自然な対話を観察し、新たなインサイトを得るための環境。制御された変数のなかで知性のゆらぎを捉えます。
                    </p>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-10">
                        <div class="bg-surface-container-high/40 p-5 rounded-2xl hover:bg-surface-container-high transition-colors group">
                            <span class="material-symbols-outlined text-primary mb-3 block group-hover:scale-110 transition-transform" data-icon="shuffle">shuffle</span>
                            <h3 class="font-bold text-white mb-2">ランダム会話</h3>
                            <p class="text-xs text-on-surface-variant leading-relaxed">予期せぬ対話から生まれる創造的な発想。</p>
                        </div>
                        <div class="bg-surface-container-high/40 p-5 rounded-2xl hover:bg-surface-container-high transition-colors group border border-primary/20">
                            <span class="material-symbols-outlined text-primary mb-3 block group-hover:scale-110 transition-transform" data-icon="tag">tag</span>
                            <h3 class="font-bold text-white mb-2">話題指定</h3>
                            <p class="text-xs text-on-surface-variant leading-relaxed">特定のテーマに焦点を当てた深い議論。</p>
                        </div>
                        <div class="bg-surface-container-high/40 p-5 rounded-2xl hover:bg-surface-container-high transition-colors group">
                            <span class="material-symbols-outlined text-primary mb-3 block group-hover:scale-110 transition-transform" data-icon="forum">forum</span>
                            <h3 class="font-bold text-white mb-2">議論</h3>
                            <p class="text-xs text-on-surface-variant leading-relaxed">複数の視点が対立し、真理を導き出す。</p>
                        </div>
                    </div>

                    <div class="mt-auto">
                        <a href="/config?mode=observation" class="block text-center w-full py-4 bg-gradient-to-r from-primary/80 to-primary text-on-primary rounded-xl font-bold hover:shadow-[0_0_20px_rgba(129,236,255,0.3)] transition-all">
                            観察を開始する
                        </a>
                    </div>
                </div>
            </div>
        </div>

        <!-- Problem Solving Mode -->
        <div class="lg:col-span-5 flex flex-col">
            <div class="bg-surface-container-low/60 backdrop-blur-xl p-8 rounded-3xl border border-outline-variant/10 relative overflow-hidden flex-1 flex flex-col bg-gradient-to-br from-secondary-container/10 to-transparent">
                <div class="absolute top-0 right-0 p-8">
                    <span class="material-symbols-outlined text-secondary text-5xl opacity-20" data-icon="psychology">psychology</span>
                </div>

                <div class="relative z-10">
                    <span class="inline-block px-3 py-1 rounded-full bg-secondary/10 text-secondary text-[10px] font-bold tracking-widest uppercase mb-4">Execution</span>
                    <h2 class="text-3xl font-headline font-bold text-white mb-4">問題解決モード</h2>
                    <p class="text-on-surface-variant mb-8 text-sm leading-relaxed">
                        明確な目標に向け、最適なAIが連携して解決策を提示。複雑なタスクを効率的に分解し、実行に移します。
                    </p>

                    <div class="space-y-4 mb-10">
                        <div class="flex items-center gap-4 bg-surface-container-high/40 p-6 rounded-2xl border border-secondary/10 hover:border-secondary/30 transition-all cursor-pointer group">
                            <div class="w-12 h-12 rounded-full bg-secondary/20 flex items-center justify-center text-secondary group-hover:bg-secondary group-hover:text-white transition-all">
                                <span class="material-symbols-outlined" data-icon="auto_awesome">auto_awesome</span>
                            </div>
                            <div>
                                <h3 class="font-bold text-white text-lg">Autoモード</h3>
                                <p class="text-xs text-on-surface-variant">システムが最適なAIを自動で選定し自律解決。</p>
                            </div>
                            <span class="material-symbols-outlined ml-auto text-slate-600" data-icon="chevron_right">chevron_right</span>
                        </div>

                        <div class="flex items-center gap-4 bg-surface-container-high/40 p-6 rounded-2xl border border-secondary/10 hover:border-secondary/30 transition-all cursor-pointer group">
                            <div class="w-12 h-12 rounded-full bg-secondary/20 flex items-center justify-center text-secondary group-hover:bg-secondary group-hover:text-white transition-all">
                                <span class="material-symbols-outlined" data-icon="person_search">person_search</span>
                            </div>
                            <div>
                                <h3 class="font-bold text-white text-lg">Selfモード</h3>
                                <p class="text-xs text-on-surface-variant">ユーザーが直接AIを指揮し、プロセスを完全制御。</p>
                            </div>
                            <span class="material-symbols-outlined ml-auto text-slate-600" data-icon="chevron_right">chevron_right</span>
                        </div>
                    </div>

                    <div class="mt-auto">
                        <a href="/config?mode=problem_solving" class="block text-center w-full py-4 border-2 border-secondary text-secondary rounded-xl font-bold hover:bg-secondary hover:text-white transition-all">
                            New Session
                        </a>
                    </div>
                </div>
            </div>
        </div>

        <!-- Auxiliary Data Row -->
        <div class="lg:col-span-4 bg-surface-container-low/60 backdrop-blur-xl p-6 rounded-3xl border border-outline-variant/10">
            <div class="flex justify-between items-center mb-6">
                <h4 class="font-bold text-white">システム負荷</h4>
                <span class="text-xs text-primary font-bold">Stable</span>
            </div>
            <div class="h-32 flex items-end gap-1 px-2">
                <div class="flex-1 bg-primary/20 h-[30%] rounded-t-sm"></div>
                <div class="flex-1 bg-primary/20 h-[45%] rounded-t-sm"></div>
                <div class="flex-1 bg-primary/20 h-[35%] rounded-t-sm"></div>
                <div class="flex-1 bg-primary/40 h-[60%] rounded-t-sm"></div>
                <div class="flex-1 bg-primary/60 h-[80%] rounded-t-sm animate-pulse"></div>
                <div class="flex-1 bg-primary/30 h-[50%] rounded-t-sm"></div>
                <div class="flex-1 bg-primary/20 h-[40%] rounded-t-sm"></div>
                <div class="flex-1 bg-primary/40 h-[70%] rounded-t-sm"></div>
                <div class="flex-1 bg-primary/20 h-[45%] rounded-t-sm"></div>
            </div>
            <p class="mt-4 text-xs text-on-surface-variant text-center">リアルタイム推論コアの稼働状況</p>
        </div>

        <div class="lg:col-span-8 bg-surface-container-low/60 backdrop-blur-xl p-6 rounded-3xl border border-outline-variant/10 flex flex-col sm:flex-row items-center justify-between gap-4">
            <div class="flex gap-6 items-center">
                <div class="flex -space-x-4">
                    <div class="w-12 h-12 rounded-full border-2 border-surface bg-blue-500/20 flex items-center justify-center">
                        <span class="material-symbols-outlined text-blue-400">smart_toy</span>
                    </div>
                    <div class="w-12 h-12 rounded-full border-2 border-surface bg-purple-500/20 flex items-center justify-center">
                        <span class="material-symbols-outlined text-purple-400">psychology_alt</span>
                    </div>
                    <div class="w-12 h-12 rounded-full border-2 border-surface bg-teal-500/20 flex items-center justify-center">
                        <span class="material-symbols-outlined text-teal-400">psychology</span>
                    </div>
                </div>
                <div>
                    <h4 class="font-bold text-white">待機中のエージェント</h4>
                    <p class="text-xs text-on-surface-variant">32の専門特化AIが即時割り当て可能です</p>
                </div>
            </div>
            <button class="text-sm font-bold text-primary flex items-center gap-2 hover:gap-3 transition-all">
                エージェント管理 <span class="material-symbols-outlined text-sm" data-icon="arrow_forward">arrow_forward</span>
            </button>
        </div>
    </div>
</div>
