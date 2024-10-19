// src/utils.js
async function fetchWasmFile(url) {
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error("Failed to fetch WASM file");
    }
    return await response.arrayBuffer();
}

module.exports = {
    fetchWasmFile
};