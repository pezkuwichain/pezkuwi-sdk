const TelegramBot = require('node-telegram-bot-api');

const token = '8425626468:AAELzRtA-eb8EtbO0SbNNim6-R0oRYDsYBU';
const bot = new TelegramBot(token, {polling: true});

bot.onText(/\/start/, (msg) => {
    const chatId = msg.chat.id;
    bot.sendMessage(chatId, '🏛️ TEST: DKS Bot is working!');
    console.log('Message received from:', msg.from.username);
});

console.log('🔧 Test bot started...');
