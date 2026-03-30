<script>
    import { onMount } from 'svelte';
</script>

<svelte:head>
    <title>Session History | Machina Polis</title>
</svelte:head>

<div class="px-6 md:px-12 max-w-7xl mx-auto pb-24 lg:pb-12">
    <!-- Header & Search -->
    <div class="flex flex-col md:flex-row md:items-end justify-between gap-6 mb-12">
        <div>
            <h1 class="text-5xl font-headline font-bold text-on-surface tracking-tighter mb-2">セッション履歴</h1>
            <p class="text-on-surface-variant font-label text-sm uppercase tracking-widest">Archive of multi-agent interactions</p>
        </div>

        <div class="flex flex-col sm:flex-row gap-4 w-full md:w-auto">
            <div class="relative group flex-1 sm:w-80">
                <span class="material-symbols-outlined absolute left-4 top-1/2 -translate-y-1/2 text-outline group-focus-within:text-primary transition-colors">search</span>
                <input type="text" placeholder="履歴を検索..." class="w-full bg-surface-container-high border-none rounded-xl py-3 pl-12 pr-4 text-on-surface placeholder:text-outline focus:ring-2 focus:ring-primary/30 transition-all" />
            </div>
            <button class="flex items-center justify-center gap-2 px-6 py-3 bg-surface-container-highest text-on-surface rounded-xl hover:bg-surface-bright transition-colors">
                <span class="material-symbols-outlined">filter_list</span>
                <span class="font-label text-sm font-semibold">フィルター</span>
            </button>
        </div>
    </div>

    <!-- Bento Grid History Layout -->
    <div class="grid grid-cols-1 gap-6">
        <!-- History Item 1 -->
        <div class="bg-surface-container-low/60 backdrop-blur-xl rounded-2xl p-6 flex flex-col md:flex-row gap-6 items-start bento-card border border-outline-variant/10 hover:bg-surface-container/60 transition-colors">
            <div class="flex-shrink-0 flex flex-col items-center justify-center w-24 h-24 bg-surface-container-highest rounded-2xl border border-outline-variant/20">
                <span class="text-xs font-label text-outline uppercase">10月24日</span>
                <span class="text-2xl font-headline font-bold text-primary">14:20</span>
            </div>

            <div class="flex-grow space-y-4">
                <div class="flex flex-wrap items-center gap-3">
                    <span class="px-3 py-1 bg-primary/10 text-primary text-[10px] font-bold rounded-full uppercase tracking-tighter border border-primary/20">議論</span>
                    <h3 class="text-xl font-headline font-bold text-on-surface">量子コンピューティングの倫理的影響</h3>
                </div>

                <p class="text-on-surface-variant text-sm leading-relaxed max-w-3xl">
                    AIエージェント間での対話により、スケーラビリティとデータプライバシーのトレードオフに関する新たな視点が提示されました。最終的に、分散型検証プロトコルの導入が最適解として合意されました。
                </p>

                <div class="flex items-center gap-4">
                    <div class="flex -space-x-2">
                        <div class="w-8 h-8 rounded-full bg-blue-500 border-2 border-surface flex items-center justify-center overflow-hidden" title="GPT-4o">
                            <span class="text-[10px] font-bold">G4</span>
                        </div>
                        <div class="w-8 h-8 rounded-full bg-purple-500 border-2 border-surface flex items-center justify-center overflow-hidden" title="Claude 3.5">
                            <span class="text-[10px] font-bold">C3</span>
                        </div>
                        <div class="w-8 h-8 rounded-full bg-teal-500 border-2 border-surface flex items-center justify-center overflow-hidden" title="Gemini Pro">
                            <span class="text-[10px] font-bold">G1</span>
                        </div>
                    </div>
                    <span class="text-xs font-label text-outline">3名のエージェントが参加</span>
                </div>

                <!-- Social Sharing / Community -->
                <div class="flex items-center gap-3 pt-2">
                    <button class="text-xs flex items-center gap-1 text-primary hover:text-primary-dim transition-colors">
                        <span class="material-symbols-outlined text-[16px]">public</span>
                        コミュニティに投稿
                    </button>
                    <button class="text-xs flex items-center gap-1 text-slate-400 hover:text-white transition-colors">
                        <span class="material-symbols-outlined text-[16px]">share</span>
                        SNS共有
                    </button>
                </div>
            </div>

            <div class="flex flex-row md:flex-col gap-2 w-full md:w-auto mt-4 md:mt-0">
                <a href="/stream/mock-id-1" class="flex-1 md:w-32 py-2.5 bg-primary text-on-primary rounded-lg font-bold text-xs flex items-center justify-center gap-2 hover:brightness-110 transition-all">
                    <span class="material-symbols-outlined text-sm">visibility</span>
                    ログを表示
                </a>
                <button class="flex-1 md:w-32 py-2.5 bg-surface-container-highest text-on-surface rounded-lg font-bold text-xs flex items-center justify-center gap-2 hover:bg-surface-bright transition-all">
                    <span class="material-symbols-outlined text-sm">play_arrow</span>
                    再開する
                </button>
                <button class="w-10 h-10 md:w-32 py-2.5 text-error-dim hover:bg-error-container/10 rounded-lg flex items-center justify-center gap-2 transition-all">
                    <span class="material-symbols-outlined text-sm">delete</span>
                    <span class="hidden md:inline font-bold text-xs">削除</span>
                </button>
            </div>
        </div>

        <!-- History Item 2 -->
        <div class="bg-surface-container-low/60 backdrop-blur-xl rounded-2xl p-6 flex flex-col md:flex-row gap-6 items-start bento-card border border-outline-variant/10 hover:bg-surface-container/60 transition-colors">
            <div class="flex-shrink-0 flex flex-col items-center justify-center w-24 h-24 bg-surface-container-highest rounded-2xl border border-outline-variant/20">
                <span class="text-xs font-label text-outline uppercase">10月23日</span>
                <span class="text-2xl font-headline font-bold text-secondary">09:45</span>
            </div>

            <div class="flex-grow space-y-4">
                <div class="flex flex-wrap items-center gap-3">
                    <span class="px-3 py-1 bg-secondary/10 text-secondary text-[10px] font-bold rounded-full uppercase tracking-tighter border border-secondary/20">Selfモード</span>
                    <h3 class="text-xl font-headline font-bold text-on-surface">新プロジェクトのコードレビューとリファクタリング</h3>
                </div>

                <p class="text-on-surface-variant text-sm leading-relaxed max-w-3xl">
                    Rustベースのマイクロサービスアーキテクチャのボトルネックを特定。メモリ管理の最適化案と、非同期処理のパターン改善について具体的なコードスニペットが生成されました。
                </p>

                <div class="flex items-center gap-4">
                    <div class="flex -space-x-2">
                        <div class="w-8 h-8 rounded-full bg-secondary border-2 border-surface flex items-center justify-center overflow-hidden" title="LLama 3">
                            <span class="text-[10px] font-bold text-white">L3</span>
                        </div>
                    </div>
                    <span class="text-xs font-label text-outline">1名のエージェントが参加</span>
                </div>
            </div>

            <div class="flex flex-row md:flex-col gap-2 w-full md:w-auto mt-4 md:mt-0">
                <a href="/stream/mock-id-2" class="flex-1 md:w-32 py-2.5 bg-primary text-on-primary rounded-lg font-bold text-xs flex items-center justify-center gap-2 hover:brightness-110 transition-all">
                    <span class="material-symbols-outlined text-sm">visibility</span>
                    ログを表示
                </a>
                <button class="flex-1 md:w-32 py-2.5 bg-surface-container-highest text-on-surface rounded-lg font-bold text-xs flex items-center justify-center gap-2 hover:bg-surface-bright transition-all">
                    <span class="material-symbols-outlined text-sm">play_arrow</span>
                    再開する
                </button>
            </div>
        </div>

    </div>

    <!-- Load More Section -->
    <div class="flex justify-center mt-8">
        <button class="group flex items-center gap-3 px-8 py-4 bg-transparent border-2 border-outline-variant hover:border-primary/50 text-on-surface rounded-full transition-all">
            <span class="font-label font-bold text-sm">さらに古い履歴を読み込む</span>
            <span class="material-symbols-outlined group-hover:translate-y-1 transition-transform">expand_more</span>
        </button>
    </div>
</div>
