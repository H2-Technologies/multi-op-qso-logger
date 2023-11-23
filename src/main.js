const invoke = window.__TAURI__.primitives.invoke;

async function invokeRust() {
    const res = await invoke('parse_csv');
    console.log(res);
    console.log('done');
}
