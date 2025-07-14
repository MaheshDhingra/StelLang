import { Client, Events, GatewayIntentBits, SlashCommandBuilder, REST, Routes, Partials, ChatInputCommandInteraction, GuildMember, Message, TextChannel, NewsChannel, ThreadChannel, AnyThreadChannel, GuildBasedChannel } from 'discord.js';
import fetch from 'node-fetch';
import * as fs from 'fs';
import path from 'path';
import { readdirSync } from 'fs';

// Read config.json
interface Config {
  token: string;
  clientId: string;
  guildId: string;
}
const config: Config = JSON.parse(fs.readFileSync('./config.json', 'utf-8'));
const { token, clientId, guildId } = config;

// --- Moderation settings ---
const BAD_WORDS = ["badword1", "badword2", "idiot", "dumb"];
const SPAM_THRESHOLD = 5; // messages per 5 seconds
const userMessageTimestamps: Record<string, number[]> = {};

// --- Welcome Message ---
const WELCOME_MESSAGES = [
  "Welcome to the StelLang Discord, {user}! 🚀",
  "Hey {user}, glad to have you in the StelLang community! 🌟",
  "{user} just joined. Everyone, look busy! 😄",
  "A wild {user} appeared! Welcome! 🦄"
];

// --- Documentation/Help/Error Data ---
const DOC_LINK = "https://github.com/maheshdhingra/stellang";
const COMMON_ERRORS: Record<string, string> = {
  "syntax": "Check for missing semicolons, brackets, or typos in your code.",
  "module not found": "Make sure the module is installed and the import path is correct.",
  "permission": "Check your permissions and try running as admin/root if needed."
};

// Command interface
interface Command {
  data: SlashCommandBuilder;
  execute: (interaction: ChatInputCommandInteraction) => Promise<void>;
}

// Load commands dynamically from commands/ directory
const commandsDir = path.join(__dirname, 'commands');
const commandFiles = readdirSync(commandsDir).filter(file => file.endsWith('.ts') || file.endsWith('.js'));
const commandMap: Map<string, Command> = new Map();
const commandDataArray = [];
for (const file of commandFiles) {
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const command: Command = require(path.join(commandsDir, file));
  commandMap.set(command.data.name, command);
  commandDataArray.push(command.data.toJSON());
}

const rest = new REST({ version: '10' }).setToken(token);

(async () => {
  try {
    console.log(`Started refreshing ${commandDataArray.length} application (/) commands.`);
    const data = await rest.put(
      Routes.applicationGuildCommands(clientId, guildId),
      { body: commandDataArray },
    );
    console.log(`Successfully reloaded ${(data as any[]).length} application (/) commands.`);
  } catch (error) {
    console.error(error);
  }
})();

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
    GatewayIntentBits.GuildMembers
  ],
  partials: [Partials.Message, Partials.Channel, Partials.GuildMember]
});

// --- Welcome Message Handler ---
client.on(Events.GuildMemberAdd, (member: GuildMember) => {
  const welcome = WELCOME_MESSAGES[Math.floor(Math.random() * WELCOME_MESSAGES.length)].replace('{user}', `<@${member.id}>`);
  let channel = member.guild.systemChannel as TextChannel | NewsChannel | ThreadChannel | null;
  if (!channel) {
    channel = member.guild.channels.cache.find(
      (ch) => {
        // @ts-ignore: runtime check for send method
        return typeof (ch as any).send === 'function' && ch.permissionsFor(member.guild.members.me!).has('SendMessages');
      }
    ) as TextChannel | NewsChannel | ThreadChannel | undefined || null;
  }
  if (channel) channel.send(welcome);
});

// --- Moderation Handler ---
client.on(Events.MessageCreate, async (message: Message) => {
  if (message.author.bot) return;

  // Bad word filter
  const lower = message.content.toLowerCase();
  if (BAD_WORDS.some(word => lower.includes(word))) {
    await message.delete();
    if (message.channel.type === 0 || message.channel.type === 5 || message.channel.type === 11) {
      await (message.channel as TextChannel | NewsChannel | ThreadChannel).send(`${message.author}, please avoid using inappropriate language!`);
    }
    return;
  }

  // Spam detection
  const now = Date.now();
  if (!userMessageTimestamps[message.author.id]) userMessageTimestamps[message.author.id] = [];
  userMessageTimestamps[message.author.id].push(now);
  userMessageTimestamps[message.author.id] = userMessageTimestamps[message.author.id].filter(ts => now - ts < 5000);
  if (userMessageTimestamps[message.author.id].length > SPAM_THRESHOLD) {
    if (message.channel.type === 0 || message.channel.type === 5 || message.channel.type === 11) {
      await (message.channel as TextChannel | NewsChannel | ThreadChannel).send(`${message.author}, please do not spam!`);
    }
    userMessageTimestamps[message.author.id] = [];
  }
});

// --- Slash Command Handler ---
client.on(Events.InteractionCreate, async (interaction) => {
  if (!interaction.isChatInputCommand()) return;
  const command = commandMap.get(interaction.commandName);
  if (!command) return;
  try {
    await command.execute(interaction);
  } catch (error) {
    console.error(error);
    await interaction.reply({ content: 'There was an error while executing this command!', ephemeral: true });
  }
});

client.once(Events.ClientReady, readyClient => {
  console.log(`Ready! Logged in as ${readyClient.user.tag}`);
});

client.login(token); 