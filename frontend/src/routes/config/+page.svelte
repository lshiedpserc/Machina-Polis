<script>
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let selectedModels = $state(['gpt-4o', 'claude-3-5']);
    let selectedMode = $state('discussion');
    let topic = $state('');
    let agent1Role = $state('論理的で批判的な立場から、提案の矛盾点を指摘する。');
    let agent2Role = $state('創造的でユーザー中心の視点から、エモーショナルな価値を擁護する。');

    /** @param {string} model */
    function toggleModel(model) {
        if (selectedModels.includes(model)) {
            selectedModels = selectedModels.filter(m => m !== model);
        } else {
            selectedModels = [...selectedModels, model];
        }
    }

    async function startSession() {
        if (!topic) {
            alert('議題・話題を入力してください');
            return;
        }

        try {
            const roles = {};
            if (selectedModels.includes('gpt-4o')) roles['gpt-4o'] = agent1Role;
            if (selectedModels.includes('claude-3-5')) roles['claude-3-5'] = agent2Role;
            // Gemini doesn't have a bound role input right now, keeping simple

            const response = await fetch('http://localhost:3000/api/sessions', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    topic,
                    mode: selectedMode,
                    models: selectedModels,
                    roles
                })
            });

            if (!response.ok) {
                throw new Error('Failed to create session');
            }

            const data = await response.json();

            // Start the session right away
            await fetch(`http://localhost:3000/api/sessions/${data.session_id}/start`, {
                method: 'POST'
            });

            goto(`/stream/${data.session_id}`);
        } catch (e) {
            console.error(e);
            alert('エラーが発生しました: ' + (e instanceof Error ? e.message : 'Unknown error'));
        }
    }
</script>

<svelte:head>
    <title>Configuration | Machina Polis</title>
</svelte:head>

<div class="px-6 md:px-12 max-w-5xl mx-auto pb-24 lg:pb-12">
    <!-- Header Section -->
    <header class="mb-12">
        <h1 class="text-4xl md:text-5xl font-headline font-bold text-on-surface tracking-tight mb-2">セッション設定</h1>
        <p class="text-on-surface-variant font-label max-w-2xl">複数のAIエージェントを構成し、目的や対話形式を選択してください。あなたの意図をデジタル・キュレーターが具現化します。</p>
    </header>

    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start">
        <!-- Main Form Column -->
        <div class="lg:col-span-8 space-y-8">
            <!-- Section: AI API Selection -->
            <section class="p-8 rounded-2xl bg-surface-container-low/80 backdrop-blur-xl relative overflow-hidden group">
                <div class="absolute top-0 right-0 w-32 h-32 bg-primary/5 rounded-full blur-3xl -mr-16 -mt-16"></div>

                <h3 class="flex items-center gap-3 text-lg font-headline font-semibold text-primary mb-6">
                    <span class="material-symbols-outlined">hub</span>
                    AIモデルの選択
                </h3>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <!-- AI Card 1 -->
                    <div class="relative">
                        <input type="checkbox" id="ai-1" class="hidden peer" checked={selectedModels.includes('gpt-4o')} onchange={() => toggleModel('gpt-4o')}/>
                        <label for="ai-1" class="flex items-center p-4 rounded-xl border border-outline-variant bg-surface-container-high cursor-pointer transition-all hover:border-primary/50 peer-checked:border-primary peer-checked:bg-primary/10">
                            <div class="w-10 h-10 rounded-lg bg-blue-500/20 flex items-center justify-center mr-4">
                                <span class="material-symbols-outlined text-blue-400">psychology</span>
                            </div>
                            <div class="flex-1">
                                <p class="font-bold text-on-surface">GPT-4 Omni</p>
                                <p class="text-xs text-on-surface-variant">高精度の推論と分析</p>
                            </div>
                            <span class="material-symbols-outlined text-primary opacity-0 peer-checked:opacity-100 transition-opacity">check_circle</span>
                        </label>
                    </div>

                    <!-- AI Card 2 -->
                    <div class="relative">
                        <input type="checkbox" id="ai-2" class="hidden peer" checked={selectedModels.includes('claude-3-5')} onchange={() => toggleModel('claude-3-5')}/>
                        <label for="ai-2" class="flex items-center p-4 rounded-xl border border-outline-variant bg-surface-container-high cursor-pointer transition-all hover:border-primary/50 peer-checked:border-primary peer-checked:bg-primary/10">
                            <div class="w-10 h-10 rounded-lg bg-purple-500/20 flex items-center justify-center mr-4">
                                <span class="material-symbols-outlined text-purple-400">auto_awesome</span>
                            </div>
                            <div class="flex-1">
                                <p class="font-bold text-on-surface">Claude 3.5 Sonnet</p>
                                <p class="text-xs text-on-surface-variant">ニュアンス豊かな表現</p>
                            </div>
                            <span class="material-symbols-outlined text-primary opacity-0 peer-checked:opacity-100 transition-opacity">check_circle</span>
                        </label>
                    </div>

                    <!-- AI Card 3 -->
                    <div class="relative">
                        <input type="checkbox" id="ai-3" class="hidden peer" checked={selectedModels.includes('gemini-1-5')} onchange={() => toggleModel('gemini-1-5')}/>
                        <label for="ai-3" class="flex items-center p-4 rounded-xl border border-outline-variant bg-surface-container-high cursor-pointer transition-all hover:border-primary/50 peer-checked:border-primary peer-checked:bg-primary/10">
                            <div class="w-10 h-10 rounded-lg bg-teal-500/20 flex items-center justify-center mr-4">
                                <span class="material-symbols-outlined text-teal-400">bolt</span>
                            </div>
                            <div class="flex-1">
                                <p class="font-bold text-on-surface">Gemini 1.5 Pro</p>
                                <p class="text-xs text-on-surface-variant">広大なコンテキスト理解</p>
                            </div>
                            <span class="material-symbols-outlined text-primary opacity-0 peer-checked:opacity-100 transition-opacity">check_circle</span>
                        </label>
                    </div>
                </div>
            </section>

            <!-- Section: Mode Selection -->
            <section class="p-8 rounded-2xl bg-surface-container-low/80 backdrop-blur-xl">
                <h3 class="flex items-center gap-3 text-lg font-headline font-semibold text-secondary mb-6">
                    <span class="material-symbols-outlined">settings_input_component</span>
                    モード選択
                </h3>

                <div class="space-y-8">
                    <!-- Observation Group -->
                    <div>
                        <p class="text-sm font-label text-on-surface-variant mb-4 uppercase tracking-widest">Observation — 観察・生成</p>
                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
                            <div class="relative">
                                <input type="radio" id="mode-random" name="mode" value="random" bind:group={selectedMode} class="hidden peer" />
                                <label for="mode-random" class="block p-4 rounded-xl border border-outline-variant bg-surface-container-high text-center cursor-pointer transition-all peer-checked:border-secondary peer-checked:bg-secondary/10 peer-checked:text-secondary">
                                    <span class="block text-on-surface font-bold">Random</span>
                                </label>
                            </div>
                            <div class="relative">
                                <input type="radio" id="mode-topic" name="mode" value="topic" bind:group={selectedMode} class="hidden peer" />
                                <label for="mode-topic" class="block p-4 rounded-xl border border-outline-variant bg-surface-container-high text-center cursor-pointer transition-all peer-checked:border-secondary peer-checked:bg-secondary/10 peer-checked:text-secondary">
                                    <span class="block text-on-surface font-bold">Topic</span>
                                </label>
                            </div>
                            <div class="relative">
                                <input type="radio" id="mode-discussion" name="mode" value="discussion" bind:group={selectedMode} class="hidden peer" />
                                <label for="mode-discussion" class="block p-4 rounded-xl border border-outline-variant bg-surface-container-high text-center cursor-pointer transition-all peer-checked:border-secondary peer-checked:bg-secondary/10 peer-checked:text-secondary">
                                    <span class="block text-on-surface font-bold">Discussion</span>
                                </label>
                            </div>
                        </div>
                    </div>

                    <!-- Problem Solving Group -->
                    <div>
                        <p class="text-sm font-label text-on-surface-variant mb-4 uppercase tracking-widest">Problem Solving — 課題解決</p>
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
                            <div class="relative">
                                <input type="radio" id="mode-auto" name="mode" value="auto" bind:group={selectedMode} class="hidden peer" />
                                <label for="mode-auto" class="block p-4 rounded-xl border border-outline-variant bg-surface-container-high text-center cursor-pointer transition-all peer-checked:border-tertiary peer-checked:bg-tertiary/10 peer-checked:text-tertiary">
                                    <span class="block text-on-surface font-bold">Auto</span>
                                </label>
                            </div>
                            <div class="relative">
                                <input type="radio" id="mode-self" name="mode" value="self" bind:group={selectedMode} class="hidden peer" />
                                <label for="mode-self" class="block p-4 rounded-xl border border-outline-variant bg-surface-container-high text-center cursor-pointer transition-all peer-checked:border-tertiary peer-checked:bg-tertiary/10 peer-checked:text-tertiary">
                                    <span class="block text-on-surface font-bold">Self</span>
                                </label>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            <!-- Section: Topic Input -->
            <section class="p-8 rounded-2xl bg-surface-container-low/80 backdrop-blur-xl">
                <h3 class="flex items-center gap-3 text-lg font-headline font-semibold text-primary mb-6">
                    <span class="material-symbols-outlined">edit_note</span>
                    議題・話題の入力
                </h3>
                <div class="relative">
                    <textarea bind:value={topic} class="w-full bg-surface-container-high border border-outline-variant rounded-xl p-5 text-on-surface placeholder:text-on-surface-variant focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all min-h-[120px]" placeholder="AIたちに議論させたいテーマを具体的に入力してください..."></textarea>
                    <div class="absolute bottom-4 right-4 text-[10px] text-on-surface-variant font-label">MAX 2000 CHARACTERS</div>
                </div>
            </section>

            <!-- Section: Role Assignment (Visible if Discussion is selected) -->
            {#if selectedMode === 'discussion'}
            <section class="p-8 rounded-2xl bg-surface-container-low/80 backdrop-blur-xl border-l-4 border-secondary/50" id="role-assignment">
                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between mb-8 gap-4">
                    <h3 class="flex items-center gap-3 text-lg font-headline font-semibold text-secondary">
                        <span class="material-symbols-outlined">diversity_3</span>
                        AIロール・立ち位置の設定
                    </h3>
                    <span class="px-3 py-1 bg-secondary/20 text-secondary text-[10px] font-bold rounded-full tracking-wider uppercase">Active: Discussion Mode</span>
                </div>

                <div class="space-y-6">
                    {#if selectedModels.includes('gpt-4o')}
                    <!-- Agent Role 1 -->
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 items-center">
                        <div class="flex items-center gap-3">
                            <div class="w-2 h-2 rounded-full bg-blue-500"></div>
                            <span class="font-bold text-on-surface">GPT-4 Omni</span>
                        </div>
                        <div class="md:col-span-3">
                            <input bind:value={agent1Role} type="text" class="w-full bg-surface-container-high border border-outline-variant rounded-lg px-4 py-2 text-sm text-on-surface focus:border-secondary outline-none" />
                        </div>
                    </div>
                    {/if}

                    {#if selectedModels.includes('claude-3-5')}
                    <!-- Agent Role 2 -->
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 items-center">
                        <div class="flex items-center gap-3">
                            <div class="w-2 h-2 rounded-full bg-purple-500"></div>
                            <span class="font-bold text-on-surface">Claude 3.5</span>
                        </div>
                        <div class="md:col-span-3">
                            <input bind:value={agent2Role} type="text" class="w-full bg-surface-container-high border border-outline-variant rounded-lg px-4 py-2 text-sm text-on-surface focus:border-secondary outline-none" />
                        </div>
                    </div>
                    {/if}

                    {#if selectedModels.includes('gemini-1-5')}
                    <!-- Agent Role 3 -->
                    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 items-center">
                        <div class="flex items-center gap-3">
                            <div class="w-2 h-2 rounded-full bg-teal-500"></div>
                            <span class="font-bold text-on-surface">Gemini 1.5 Pro</span>
                        </div>
                        <div class="md:col-span-3">
                            <input type="text" class="w-full bg-surface-container-high border border-outline-variant rounded-lg px-4 py-2 text-sm text-on-surface focus:border-secondary outline-none" placeholder="Geminiの役割を入力..." />
                        </div>
                    </div>
                    {/if}
                </div>
            </section>
            {/if}

            <!-- Bottom Action -->
            <div class="pt-8 flex justify-end">
                <button onclick={startSession} class="px-12 py-5 bg-gradient-to-r from-primary to-secondary text-on-primary-container font-headline font-bold text-xl rounded-xl shadow-[0_0_40px_rgba(129,236,255,0.3)] hover:shadow-[0_0_60px_rgba(193,128,255,0.4)] transition-all scale-100 hover:scale-[1.02] active:scale-95 flex items-center gap-3">
                    <span>開始する</span>
                    <span class="material-symbols-outlined">rocket_launch</span>
                </button>
            </div>
        </div>

        <!-- Sidebar / Visual Anchor Column -->
        <div class="lg:col-span-4 space-y-8">
            <!-- Summary Card -->
            <div class="rounded-2xl bg-surface-container p-6 border border-outline-variant/30">
                <h4 class="text-xs font-label text-on-surface-variant tracking-[0.2em] mb-6 uppercase">Session Preview</h4>

                <div class="relative w-full aspect-square rounded-xl overflow-hidden mb-6 bg-gradient-to-br from-blue-900 to-purple-900 flex items-center justify-center">
                    <span class="material-symbols-outlined text-white/20 text-6xl">blur_on</span>
                    <div class="absolute inset-0 bg-gradient-to-t from-slate-950 to-transparent opacity-60"></div>
                    <div class="absolute bottom-4 left-4">
                        <span class="text-xs font-label text-white/70">Estimated Tokens</span>
                        <p class="text-xl font-headline font-bold text-white">4,200 ~ 8,500</p>
                    </div>
                </div>

                <div class="space-y-4">
                    <div class="flex justify-between items-center py-2 border-b border-outline-variant/20">
                        <span class="text-sm text-on-surface-variant">エージェント数</span>
                        <span class="text-sm font-bold text-on-surface">{selectedModels.length}</span>
                    </div>
                    <div class="flex justify-between items-center py-2 border-b border-outline-variant/20">
                        <span class="text-sm text-on-surface-variant">想定所要時間</span>
                        <span class="text-sm font-bold text-on-surface">~ 5分</span>
                    </div>
                    <div class="flex justify-between items-center py-2 border-b border-outline-variant/20">
                        <span class="text-sm text-on-surface-variant">並列処理</span>
                        <span class="text-sm font-bold text-tertiary">ENABLED</span>
                    </div>
                </div>
            </div>

            <!-- Tip Card -->
            <div class="rounded-2xl bg-primary/5 p-6 border border-primary/20">
                <div class="flex items-center gap-2 mb-3 text-primary">
                    <span class="material-symbols-outlined text-sm">info</span>
                    <span class="text-xs font-bold tracking-wider">PRO TIP</span>
                </div>
                <p class="text-xs leading-relaxed text-on-surface-variant">
                    「Discussion」モードでは、AI同士に異なる性格を付与することで、より多角的な視点からの解決策が得られます。例えば、一方は「冷徹な会計士」、もう一方は「熱血なデザイナー」といった設定が効果的です。
                </p>
            </div>
        </div>
    </div>
</div>
