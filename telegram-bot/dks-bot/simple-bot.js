const TelegramBot = require('node-telegram-bot-api');
const token = '8425626468:AAELzRtA-eb8EtbO0SbNNim6-R0oRYDsYBU';
const bot = new TelegramBot(token, {polling: true});

bot.on('message', (msg) => {
  console.log('Message received:', msg.text);
  bot.sendMessage(msg.chat.id, '🏛️ DKS Bot is working!');
});

console.log('Simple bot started...');
