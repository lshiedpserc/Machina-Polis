<script>
    import { page } from '$app/state';
    import { onMount, onDestroy } from 'svelte';

    /** @type {any[]} */
    let messages = $state([]);
    /** @type {WebSocket | null} */
    let socket = $state(null);
    let sessionStatus = $state('connecting'); // connecting, active, paused, stopped
    let isHumanTurn = $state(false);
    let humanInput = $state('');

    /** @type {any} */
    let session = $state(null);
    /** @type {string | null} */
    let error = $state(null);

    onMount(async () => {
        const sessionId = page.params.id;

        try {
            // Fetch initial session state
            const res = await fetch(`http://localhost:3000/api/sessions/${sessionId}`);
            if (res.ok) {
                session = await res.json();
                messages = session.messages || [];
                sessionStatus = session.status || 'connecting';
            } else {
                error = "セッションが見つかりませんでした";
                return;
            }
        } catch(e) {
            console.error(e);
            error = "サーバーへの接続に失敗しました";
            return;
        }

        // Connect to WebSocket
        socket = new WebSocket(`ws://localhost:3000/ws/${sessionId}`);

        socket.onmessage = (/** @type {MessageEvent} */ event) => {
            try {
                const newMsg = JSON.parse(event.data);
                // Check if we already have this message (by id) to avoid duplicates from initial fetch vs WS
                if (!messages.find(m => m.id === newMsg.id)) {
                    messages = [...messages, newMsg];
                }
            } catch (e) {
                console.error("Failed to parse message", e);
            }
        };

        socket.onopen = () => {
            console.log("WebSocket connected");
            sessionStatus = 'active';
        };

        socket.onclose = () => {
            console.log("WebSocket disconnected");
            sessionStatus = 'stopped';
        };
    });

    onDestroy(() => {
        if (socket) {
            socket.close();
        }
    });

    function handleHumanSubmit() {
        if (!humanInput.trim()) return;

        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.send(JSON.stringify({ content: humanInput }));
            humanInput = '';
            isHumanTurn = false;
        }
    }
</script>

<svelte:head>
    <title>Chat Stream | Machina Polis</title>
</svelte:head>

<!-- We override the layout for the chat screen to fit the whole window -->
<div class="fixed inset-0 pt-16 lg:ml-64 flex flex-col bg-surface z-20">
    <!-- Chat Header / Status Area -->
    <div class="h-14 border-b border-outline-variant/10 flex items-center justify-between px-8 bg-surface-container/30 backdrop-blur-sm flex-shrink-0">
        <div class="flex items-center gap-4 truncate">
            <h1 class="font-headline font-bold text-lg text-on-surface truncate">{session ? session.title : 'Loading...'}</h1>

            <div class="flex items-center gap-2 bg-surface-container-highest px-3 py-1 rounded-full flex-shrink-0">
                <span class="material-symbols-outlined text-sm text-tertiary-dim" style="font-variation-settings: 'FILL' 1;">visibility</span>
                <span class="text-[11px] font-label font-bold text-on-surface-variant">{sessionStatus}</span>
            </div>
        </div>

        <div class="flex -space-x-2">
            <div class="w-8 h-8 rounded-full ring-2 ring-surface bg-primary p-0.5 flex items-center justify-center">
                <span class="material-symbols-outlined text-xs text-on-primary">psychology</span>
            </div>
            <div class="w-8 h-8 rounded-full ring-2 ring-surface bg-secondary p-0.5 flex items-center justify-center">
                <span class="material-symbols-outlined text-xs text-white">auto_awesome</span>
            </div>
        </div>
    </div>

    <div class="flex flex-1 overflow-hidden relative">
        <!-- Chat Stream -->
        <div class="flex-1 overflow-y-auto p-4 md:p-8 space-y-12 pb-32">
            <!-- Message List -->
            {#each messages as msg}
                {#if msg.role === 'system'}
                    <div class="flex justify-center">
                        <span class="px-4 py-1 rounded-full bg-surface-container-highest text-xs text-on-surface-variant font-label tracking-widest">{msg.content}</span>
                    </div>
                {:else if msg.role === 'human'}
                    <div class="flex gap-4 md:gap-6 max-w-4xl group ml-auto flex-row-reverse">
                        <div class="flex-shrink-0">
                            <div class="w-10 h-10 md:w-12 md:h-12 rounded-2xl bg-surface-container-highest flex items-center justify-center border border-outline-variant/30">
                                <span class="material-symbols-outlined text-on-surface">person</span>
                            </div>
                        </div>
                        <div class="flex-1 space-y-3 text-right">
                            <div class="flex items-center justify-end gap-3">
                                <span class="font-headline font-bold text-on-surface">You</span>
                            </div>
                            <div class="bg-surface-container-high p-4 md:p-6 rounded-2xl rounded-tr-none border border-outline-variant/30 text-on-surface leading-relaxed shadow-xl text-left inline-block max-w-full whitespace-pre-wrap">{msg.content}</div>
                        </div>
                    </div>
                {:else}
                    <!-- AI Message (We will style based on agent later) -->
                    <div class="flex gap-4 md:gap-6 max-w-4xl group">
                        <div class="flex-shrink-0">
                            <div class="w-10 h-10 md:w-12 md:h-12 rounded-2xl bg-primary-container/20 flex items-center justify-center border border-primary/30 relative">
                                <span class="material-symbols-outlined text-primary">psychology</span>
                            </div>
                        </div>
                        <div class="flex-1 space-y-3">
                            <div class="flex items-center gap-3">
                                <span class="font-headline font-bold text-primary">{msg.agent_name || 'AI'}</span>
                            </div>
                            <div class="bg-surface-container-high/50 p-4 md:p-6 rounded-2xl rounded-tl-none border-l-2 border-primary/50 text-on-surface leading-relaxed shadow-xl max-w-full overflow-x-auto whitespace-pre-wrap">
                                {msg.content}
                            </div>
                        </div>
                    </div>
                {/if}
            {/each}

        </div>

        <!-- Right Sidebar: Participants (Desktop only) -->
        <aside class="hidden xl:flex w-80 border-l border-outline-variant/10 bg-slate-950/40 backdrop-blur-md flex-col p-6 space-y-8 overflow-y-auto shrink-0 pb-24">
            <div>
                <h3 class="font-headline font-bold text-sm text-on-surface flex items-center gap-2 mb-6">
                    <span class="material-symbols-outlined text-primary text-sm">groups</span>
                    アクティブ・エージェント
                </h3>

                <div class="space-y-4">
                    {#if session}
                        {#each session.agents as agent}
                        <div class="bg-surface-container-high/40 p-4 rounded-xl border-l-4 border-primary">
                            <div class="flex items-center gap-3 mb-2">
                                <span class="font-bold text-xs text-on-surface">{agent.name}</span>
                            </div>
                            {#if agent.role}
                            <p class="text-[11px] text-on-surface-variant leading-relaxed">{agent.role}</p>
                            {/if}
                        </div>
                        {/each}
                    {/if}
                </div>
            </div>

            <div class="pt-4 border-t border-outline-variant/10">
                <h3 class="font-headline font-bold text-sm text-on-surface mb-4">セッション情報</h3>
                <div class="grid grid-cols-2 gap-4">
                    <div class="bg-surface-container/40 p-3 rounded-lg">
                        <span class="text-[10px] font-label text-outline block mb-1">モード</span>
                        <span class="font-headline font-bold text-primary truncate max-w-full inline-block">{session ? session.mode.replace('observation_', '').replace('problem_solving_', '') : '...'}</span>
                    </div>
                </div>
            </div>
        </aside>
    </div>

    <!-- Active Controls / Input Area (Bottom) -->
    <div class="absolute bottom-16 lg:bottom-0 w-full lg:w-[calc(100%-20rem)] xl:w-[calc(100%-20rem)] 2xl:w-auto left-0 xl:right-80 bg-slate-950/80 backdrop-blur-xl border-t border-outline-variant/20 p-4 md:p-6 z-30">

        {#if isHumanTurn}
            <!-- Human Input Mode -->
            <div class="max-w-4xl mx-auto flex gap-4 items-end">
                <div class="flex-1 relative">
                    <textarea
                        bind:value={humanInput}
                        class="w-full bg-surface-container-high border border-outline-variant rounded-xl p-4 pr-12 text-on-surface focus:border-tertiary focus:ring-1 focus:ring-tertiary outline-none resize-none max-h-32 min-h-[56px]"
                        placeholder="AIたちにメッセージを送信..."
                        onkeydown={(e) => {
                            if (e.key === 'Enter' && !e.shiftKey) {
                                e.preventDefault();
                                handleHumanSubmit();
                            }
                        }}
                    ></textarea>
                </div>
                <button onclick={handleHumanSubmit} disabled={!humanInput.trim()} class="w-14 h-14 rounded-xl bg-tertiary text-on-primary-container flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed hover:bg-tertiary-dim transition-colors flex-shrink-0">
                    <span class="material-symbols-outlined">send</span>
                </button>
            </div>
        {:else}
            <!-- Current Mode: Observation Controls -->
            <div class="flex items-center justify-center gap-6 md:gap-12">
                <div class="flex flex-col items-center">
                    <button class="w-12 h-12 md:w-14 md:h-14 rounded-full bg-error-container/20 border border-error/50 flex items-center justify-center text-error hover:bg-error/20 transition-colors group mb-2">
                        <span class="material-symbols-outlined text-2xl md:text-3xl group-active:scale-90 transition-transform" style="font-variation-settings: 'FILL' 1;">stop_circle</span>
                    </button>
                    <span class="text-[9px] md:text-[10px] font-label font-bold text-error uppercase tracking-widest hidden sm:block">Stop</span>
                </div>

                <div class="h-10 w-[1px] bg-outline-variant/30"></div>

                <div class="flex flex-col items-center">
                    <button class="w-10 h-10 md:w-12 md:h-12 rounded-full bg-surface-container-highest border border-outline-variant/30 flex items-center justify-center text-on-surface hover:bg-surface-bright transition-colors group mb-2">
                        <span class="material-symbols-outlined">pause</span>
                    </button>
                    <span class="text-[9px] md:text-[10px] font-label text-on-surface-variant uppercase tracking-widest hidden sm:block">Pause</span>
                </div>

                <div class="flex flex-col items-center">
                    <button onclick={() => isHumanTurn = true} class="w-10 h-10 md:w-12 md:h-12 rounded-full bg-surface-container-highest border border-outline-variant/30 flex items-center justify-center text-on-surface hover:bg-surface-bright transition-colors group mb-2">
                        <span class="material-symbols-outlined">edit_square</span>
                    </button>
                    <span class="text-[9px] md:text-[10px] font-label text-on-surface-variant uppercase tracking-widest hidden sm:block">Take Control</span>
                </div>
            </div>
        {/if}
    </div>
</div>
