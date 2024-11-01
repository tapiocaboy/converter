const WebSocket = require('ws');

const wss = new WebSocket.Server({ port: 8080 });

wss.on('connection', ws => {
    console.log('Client connected');

    ws.on('message', message => {
        console.log('Received:', message.toString());
        ws.send(`Echo: ${message}`);
    });

    ws.on('close', () => console.log('Client disconnected'));
    ws.on('error', error => console.error('WebSocket error:', error));
});

console.log('WebSocket server running on ws://localhost:8080');