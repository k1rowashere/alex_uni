let site_root = window.location.host.split(":")[0];
window.ws = new WebSocket(`ws://${site_root}:3001`);
ws.onmessage = () => setTimeout(() => location.reload(), 100);
