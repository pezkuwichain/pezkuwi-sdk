const TelegramBot = require('node-telegram-bot-api');
const fs = require('fs');

// Bot token
const token = '8425626468:AAELzRtA-eb8EtbO0SbNNim6-R0oRYDsYBU';
const bot = new TelegramBot(token, {polling: true});

// Game data
let users = {};

// Load game data
if (fs.existsSync('users.json')) {
    users = JSON.parse(fs.readFileSync('users.json', 'utf8'));
}

function saveUsers() {
    fs.writeFileSync('users.json', JSON.stringify(users, null, 2));
}

// Kurdish historical memory photos with interactive elements
const memoryGames = [
    {
        title_ku: "Êrişa kîmyayî ya Helebceyê", // Kurmanci Latin
        title_so: "کیمیابارانی ھەڵەبجە", // Sorani Arabic  
        title_en: "Halabja Chemical Attack",
        year: "1988",
        description_ku: "Seddam Hiseyn êrişa kîmyayî li Helebceyê kir",
        description_so: "سادام حسین کیمیابارانی کردە سەر ھەڵەبجە", 
        description_en: "Saddam Hussein chemical attack on Halabja",
        interactive_ku: "Hêstiran kom bike ji bo giyanên windayî",
        interactive_so: "فرمێسک کۆبکەرەوە بۆ گیانە لەدەستچووەکان",
        interactive_en: "Collect tears for the lost souls", 
        reward: { hez: 100, pez: 50, type: "tears" },
        emoji: "😢💔"
    },
    {
        title_ku: "Enfala Kurdan", // Kurmanci Latin
        title_so: "ئەنفالی کوردەکان", // Sorani Arabic
        title_en: "Kurdish Anfal Genocide",
        year: "1986-1989",
        description_ku: "Jenosîd li dijî gelê Kurd",
        description_so: "ژەنۆساید لە دژی گەلی کورد",
        description_en: "Genocide against Kurdish people",
        interactive_ku: "Şem vêxe ji bo bîra qurbanîyan",
        interactive_so: "مۆم داگیرسێنە بۆ یادی قوربانییەکان",
        interactive_en: "Light candles for the victims",
        reward: { hez: 150, pez: 75, type: "candles" },
        emoji: "🕯️🖤"
    },
    {
        title_ku: "Şoreşa Rojava", // Kurmanci Latin
        title_so: "شۆڕشی ڕۆژاوا", // Sorani Arabic  
        title_en: "Rojava Revolution",
        year: "2012",
        description_ku: "Xweparastin û azadî li Rojhilata Kurdistanê",
        description_so: "خۆڕاگری و ئازادی لە ڕۆژاوایی کوردستان",
        description_en: "Self-defense and freedom in Western Kurdistan",
        interactive_ku: "Alan bilind bike ji bo azadiyê", 
        interactive_so: "ئاڵا ببەرەوە بۆ ئازادی",
        interactive_en: "Raise flags for freedom",
        reward: { hez: 200, pez: 100, type: "flags" },
        emoji: "🚩💪"
    },
    {
        title_ku: "Newroza Kurdî", // Kurmanci Latin
        title_so: "نەورۆزی کوردی", // Sorani Arabic
        title_en: "Kurdish Newroz", 
        year: "Ancient",
        description_ku: "Cejna bihar û jiyana Kurdan",
        description_so: "جەژنی بەهار و زیندووی کوردەکان",
        description_en: "Kurdish spring and life celebration",
        interactive_ku: "Agir vêxe ji bo Newrozê",
        interactive_so: "ئاگر داگیرسێنە بۆ نەورۆز", 
        interactive_en: "Light fires for Newroz",
        reward: { hez: 75, pez: 125, type: "fire" },
        emoji: "🔥🌸"
    }
];

// Daily Kurdish quiz questions
const dailyQuiz = [
    {
        question_ku: "Paytext ya Kurdistana Iraqê çi ye?", // Kurmanci Latin
        question_so: "پایتەختی کوردستانی عێراق چیە؟", // Sorani Arabic
        question_en: "What is the capital of Iraqi Kurdistan?",
        options: ["Hewlêr/Erbil", "Silêmanî/Sulaymaniyah", "Dihok/Duhok", "Kerkûk/Kirkuk"],
        correct: 0,
        difficulty: "easy",
        reward: { hez: 10, pez: 5 }
    },
    {
        question_ku: "Kî yekem partiya siyasî ya Kurdî damezrand?", // Kurmanci Latin
        question_so: "کە یەکەم پارتی سیاسی کوردی دامەزراند؟", // Sorani Arabic
        question_en: "Who founded the first Kurdish political party?",
        options: ["Qazi Muhammad", "Mustafa Barzani", "Ahmad Khani", "Cigerxwîn"],
        correct: 0,
        difficulty: "medium",
        reward: { hez: 25, pez: 15 }
    },
    {
        question_ku: "Komara Kurdistanê li ku hat damezrandin?", // Kurmanci Latin
        question_so: "کۆماری کوردستان لە کوێ دامەزرا؟", // Sorani Arabic
        question_en: "Where was the Republic of Kurdistan established?",
        options: ["Mahabad", "Hewlêr", "Qamişlo", "Dêrsim"],
        correct: 0,
        difficulty: "hard",
        reward: { hez: 50, pez: 25 }
    }
];

// Generate referral code
function generateReferralCode(userId) {
    return `DKS${userId}${Math.random().toString(36).substr(2, 4).toUpperCase()}`;
}

// Welcome messages
const welcomeMessage_ku = `
🏛️ Bi xêr hatî Dîjîtal Kurdistanê! 

💎 Lîstika Kom kirina Diamond
🎮 "Ew tê kirî jibîr neke"

🔹 HEZDiamond: Xalên HEZ ji bo mainnet
💠 PEZDiamond: Xalên PEZ ji bo mainnet
🎫 NFT Tickets: 201 NFT taybet

/help ji bo rêberî
`;

const welcomeMessage_so = `
🏛️ بەخێربێیت بۆ دیجیتاڵ کوردستان!

💎 یاری کۆکردنەوەی دایمۆند  
🎮 "ئەوەی پێت کراوە لەبیر مەکە"

🔹 HEZDiamond: خاڵی HEZ بۆ مەین‌نێت
💠 PEZDiamond: خاڵی PEZ بۆ مەین‌نێت
🎫 NFT Tickets: ٢٠١ NFT تایبەت

/help بۆ ڕێنمایی
`;

const welcomeMessage_en = `
🏛️ Welcome to Digital Kurdistan!

💎 Diamond Collector Game
🎮 "Never Forget What Was Done To You"

🔹 HEZDiamond: Mainnet HEZ airdrop points
💠 PEZDiamond: Mainnet PEZ airdrop points  
🎫 NFT Tickets: 201 exclusive NFTs

/help for instructions
`;

// Start command
bot.onText(/\/start(?:\s+(.+))?/, (msg, match) => {
    const chatId = msg.chat.id;
    const userId = msg.from.id;
    const username = msg.from.username || msg.from.first_name;
    const referralCode = match[1];
    
    if (!users[userId]) {
        users[userId] = {
            id: userId,
            username: username,
            chatId: chatId,
            joinDate: new Date().toISOString(),
            hezDiamond: 100, // Welcome bonus
            pezDiamond: 50,
            nftTickets: 1,
            language: 'en', // Default English, can be changed to 'ku' or 'so'
            referralCode: generateReferralCode(userId),
            referrals: [],
            referredBy: referralCode || null,
            dailyQuizCompleted: false,
            memoryGameCompleted: false,
            socialTasks: {
                twitter: false,
                telegram: false,
                youtube: false,
                instagram: false,
                facebook: false
            },
            lastPlayDate: null
        };
        
        // Referral bonuses
        if (referralCode) {
            const referrer = Object.values(users).find(u => u.referralCode === referralCode);
            if (referrer) {
                referrer.hezDiamond += 200;
                referrer.pezDiamond += 100;
                referrer.referrals.push(userId);
                users[userId].hezDiamond += 150;
                users[userId].pezDiamond += 75;
            }
        }
        
        saveUsers();
    }
    
    // Send welcome message based on language preference
    let welcomeMsg = welcomeMessage_en; // Default
    if (users[userId].language === 'ku') welcomeMsg = welcomeMessage_ku;
    if (users[userId].language === 'so') welcomeMsg = welcomeMessage_so;
    
    bot.sendMessage(chatId, welcomeMsg);
});

// Language selection
bot.onText(/\/language/, (msg) => {
    const userId = msg.from.id;
    if (!users[userId]) return;
    
    const keyboard = {
        reply_markup: {
            inline_keyboard: [
                [
                    { text: "🇹🇷 Kurmancî", callback_data: "lang_ku" },
                    { text: "🇮🇶 Soranî", callback_data: "lang_so" },
                    { text: "🇬🇧 English", callback_data: "lang_en" }
                ]
            ]
        }
    };
    
    bot.sendMessage(msg.chat.id, "Zimanê xwe hilbijêre / زمانەکەت هەڵبژێرە / Choose your language:", keyboard);
});

// Daily Memory Game
bot.onText(/\/memory/, (msg) => {
    const userId = msg.from.id;
    const user = users[userId];
    
    if (!user) {
        bot.sendMessage(msg.chat.id, "Please /start first.");
        return;
    }
    
    const today = new Date().toDateString();
    if (user.lastPlayDate === today && user.memoryGameCompleted) {
        bot.sendMessage(msg.chat.id, "🎮 You've already completed today's memory game. Come back tomorrow!");
        return;
    }
    
    // Select random memory photo
    const memoryGame = memoryGames[Math.floor(Math.random() * memoryGames.length)];
    const lang = user.language || 'en';
    
    let title, description, interactive;
    
    if (lang === 'ku') {
        title = memoryGame.title_ku;
        description = memoryGame.description_ku;
        interactive = memoryGame.interactive_ku;
    } else if (lang === 'so') {
        title = memoryGame.title_so;
        description = memoryGame.description_so; 
        interactive = memoryGame.interactive_so;
    } else {
        title = memoryGame.title_en;
        description = memoryGame.description_en;
        interactive = memoryGame.interactive_en;
    }
    
    const memoryMessage = `
${memoryGame.emoji}

**${title}**
*${memoryGame.year}*

${description}

🎯 ${interactive}

Tap to collect: +${memoryGame.reward.hez} HEZDiamond 💠 +${memoryGame.reward.pez} PEZDiamond 🔹
`;

    const collectText = lang === 'ku' ? 'Berhev bike' : lang === 'so' ? 'کۆبکەرەوە' : 'Collect';
    
    const keyboard = {
        reply_markup: {
            inline_keyboard: [
                [{ text: `${memoryGame.emoji} ${collectText}`, callback_data: `memory_${memoryGame.reward.type}` }]
            ]
        }
    };
    
    bot.sendMessage(msg.chat.id, memoryMessage, { parse_mode: 'Markdown', ...keyboard });
});

// Daily Quiz
bot.onText(/\/quiz/, (msg) => {
    const userId = msg.from.id;
    const user = users[userId];
    
    if (!user) {
        bot.sendMessage(msg.chat.id, "Please /start first.");
        return;
    }
    
    const today = new Date().toDateString();
    if (user.lastPlayDate === today && user.dailyQuizCompleted) {
        bot.sendMessage(msg.chat.id, "📚 You've completed today's quiz. Come back tomorrow!");
        return;
    }
    
    // Select random quiz question
    const quiz = dailyQuiz[Math.floor(Math.random() * dailyQuiz.length)];
    const lang = user.language || 'en';
    
    let question;
    if (lang === 'ku') question = quiz.question_ku;
    else if (lang === 'so') question = quiz.question_so;
    else question = quiz.question_en;
    
    const quizMessage = `
📚 **DAILY KURDISH QUIZ**

${question}

Difficulty: ${quiz.difficulty.toUpperCase()}
Reward: +${quiz.reward.hez} HEZDiamond 💠 +${quiz.reward.pez} PEZDiamond 🔹
`;

    const keyboard = {
        reply_markup: {
            inline_keyboard: quiz.options.map((option, index) => [
                { text: option, callback_data: `quiz_${index}_${quiz.correct}` }
            ])
        }
    };
    
    bot.sendMessage(msg.chat.id, quizMessage, { parse_mode: 'Markdown', ...keyboard });
});

// Callback query handler for language and interactions
bot.on('callback_query', (callbackQuery) => {
    const message = callbackQuery.message;
    const data = callbackQuery.data;
    const userId = callbackQuery.from.id;
    const user = users[userId];
    
    if (!user) return;
    
    // Language selection
    if (data.startsWith('lang_')) {
        const lang = data.split('_')[1];
        user.language = lang;
        saveUsers();
        
        let confirmMsg = 'Language set to English';
        if (lang === 'ku') confirmMsg = 'Ziman hate sazkirin: Kurmancî';
        if (lang === 'so') confirmMsg = 'زمان دانرا: سۆرانی';
        
        bot.editMessageText(confirmMsg, {
            chat_id: message.chat.id,
            message_id: message.message_id
        });
        return;
    }
    
    // Memory game collection
    if (data.startsWith('memory_')) {
        const today = new Date().toDateString();
        const memoryType = data.split('_')[1];
        
        const memoryGame = memoryGames.find(g => g.reward.type === memoryType);
        if (memoryGame) {
            user.hezDiamond += memoryGame.reward.hez;
            user.pezDiamond += memoryGame.reward.pez;
            user.memoryGameCompleted = true;
            user.lastPlayDate = today;
            saveUsers();
            
            const lang = user.language || 'en';
            let successMsg = `${memoryGame.emoji} Collected! +${memoryGame.reward.hez} HEZDiamond 💠 +${memoryGame.reward.pez} PEZDiamond 🔹\n\nNever forget. ❤️`;
            
            if (lang === 'ku') successMsg = `${memoryGame.emoji} Hat berhevkirin! +${memoryGame.reward.hez} HEZDiamond 💠 +${memoryGame.reward.pez} PEZDiamond 🔹\n\nJi bîr neke. ❤️`;
            if (lang === 'so') successMsg = `${memoryGame.emoji} کۆکرایەوە! +${memoryGame.reward.hez} HEZDiamond 💠 +${memoryGame.reward.pez} PEZDiamond 🔹\n\nلەبیر مەکە. ❤️`;
            
            bot.editMessageText(successMsg, {
                chat_id: message.chat.id,
                message_id: message.message_id
            });
        }
        return;
    }
    
    // Quiz answers
    if (data.startsWith('quiz_')) {
        const [, selectedAnswer, correctAnswer] = data.split('_').map(Number);
        const today = new Date().toDateString();
        
        if (selectedAnswer === correctAnswer) {
            const quiz = dailyQuiz.find(q => q.correct === correctAnswer);
            if (quiz) {
                user.hezDiamond += quiz.reward.hez;
                user.pezDiamond += quiz.reward.pez;
                user.dailyQuizCompleted = true;
                user.lastPlayDate = today;
                saveUsers();
                
                const lang = user.language || 'en';
                let successMsg = `✅ Correct! +${quiz.reward.hez} HEZDiamond 💠 +${quiz.reward.pez} PEZDiamond 🔹\n\nGreat knowledge of Kurdish history! 🎓`;
                
                if (lang === 'ku') successMsg = `✅ Rast! +${quiz.reward.hez} HEZDiamond 💠 +${quiz.reward.pez} PEZDiamond 🔹\n\nZanîna baş a dîroka Kurdan! 🎓`;
                if (lang === 'so') successMsg = `✅ ڕاست! +${quiz.reward.hez} HEZDiamond 💠 +${quiz.reward.pez} PEZDiamond 🔹\n\nزانینی باشی مێژووی کورد! 🎓`;
                
                bot.editMessageText(successMsg, {
                    chat_id: message.chat.id,
                    message_id: message.message_id
                });
            }
        } else {
            const lang = user.language || 'en';
            let failMsg = `❌ Wrong answer. The correct answer was option ${correctAnswer + 1}.\n\nKeep learning Kurdish history! 📚`;
            
            if (lang === 'ku') failMsg = `❌ Bersiva şaş. Bersiva rast ji bo ${correctAnswer + 1}.\n\nDîroka Kurdan bixwîne! 📚`;
            if (lang === 'so') failMsg = `❌ وەڵامی هەڵە. وەڵامی ڕاست ژمارە ${correctAnswer + 1} بوو.\n\nبەردەوام بە لە مێژووی کورد! 📚`;
            
            bot.editMessageText(failMsg, {
                chat_id: message.chat.id,
                message_id: message.message_id
            });
        }
        return;
    }
});

// Help command
bot.onText(/\/help/, (msg) => {
    const userId = msg.from.id;
    const user = users[userId];
    const lang = user?.language || 'en';
    
    let helpMessage = `
🏛️ **DKS DIAMOND COLLECTOR**

💎 /memory - Daily memory game
📚 /quiz - Kurdish history quiz  
💰 /wallet - Check diamond balance
👥 /refer - Referral system
📱 /social - Social media tasks
🌐 /language - Change language
📊 /stats - Game statistics

🎯 Collect diamonds for mainnet airdrop!
`;

    if (lang === 'ku') {
        helpMessage = `
🏛️ **DKS DIAMOND BERHEVKAR**

💎 /memory - Lîstika rojane ya bîranînê
📚 /quiz - Pirs û bersivên dîroka Kurdan
💰 /wallet - Balansa diamond binêre  
👥 /refer - Sîstema vexwendinê
📱 /social - Erkên medyaya civakî
🌐 /language - Zimanê biguherîne
📊 /stats - Amarên lîstikê

🎯 Diamond kom bike ji bo airdrop ya mainnet!
`;
    } else if (lang === 'so') {
        helpMessage = `
🏛️ **DKS کۆکەرەوەی دایمۆند**

💎 /memory - یاری ڕۆژانەی یادەوەری
📚 /quiz - پرسیار و وەڵامی مێژووی کورد
💰 /wallet - بالانسی دایمۆند ببینە
👥 /refer - سیستەمی دعوەت
📱 /social - ئەرکەکانی میدیای کۆمەڵایەتی  
🌐 /language - زمان بگۆڕە
📊 /stats - ئامارەکانی یاری

🎯 دایمۆند کۆبکەرەوە بۆ ئەیردرۆپی مەین‌نێت!
`;
    }
    
    bot.sendMessage(msg.chat.id, helpMessage, { parse_mode: 'Markdown' });
});

console.log('🏛️ DKS Diamond Collector Bot started! 💎');
console.log('🎮 Kurmancî: "Ew tê kirî jibîr neke"');
console.log('🎮 Soranî: "ئەوەی پێت کراوە لەبیر مەکە"');
console.log('🎮 English: "Never Forget What Was Done To You"');
