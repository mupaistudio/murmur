// Murmur - app.js

const { listen } = window.__TAURI__.event;

const statusEl = document.getElementById("status");

// 활성화 단축키 수신
listen("shortcut-activate", () => {
  console.log("shortcut-activate 수신");
  statusEl.textContent = "녹음 준비...";
});

// 삽입 단축키 수신
listen("shortcut-insert", () => {
  console.log("shortcut-insert 수신");
  statusEl.textContent = "텍스트 삽입 중...";
});

console.log("Murmur loaded");
