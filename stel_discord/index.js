const { Client, Events, GatewayIntentBits, SlashCommandBuilder, REST, Routes } = require('discord.js');
const { token, clientId, guildId } = require('./config.json');

const commands = [
	new SlashCommandBuilder().setName('ping').setDescription('Replies with pong!'),
	new SlashCommandBuilder().setName('server').setDescription('Displays server information.'),
	new SlashCommandBuilder().setName('user').setDescription('Displays user information.'),
	new SlashCommandBuilder().setName('announce').setDescription('Make an announcement.').addStringOption(option =>
		option.setName('message')
			.setDescription('The announcement message')
			.setRequired(true))
]
	.map(command => command.toJSON());

const rest = new REST({ version: '10' }).setToken(token);

(async () => {
	try {
		console.log(`Started refreshing ${commands.length} application (/) commands.`);

		const data = await rest.put(
			Routes.applicationGuildCommands(clientId, guildId),
			{ body: commands },
		);

		console.log(`Successfully reloaded ${data.length} application (/) commands.`);
	} catch (error) {
		console.error(error);
	}
})();

const client = new Client({ intents: [GatewayIntentBits.Guilds] });

client.once(Events.ClientReady, readyClient => {
	console.log(`Ready! Logged in as ${readyClient.user.tag}`);
});

client.on(Events.InteractionCreate, async interaction => {
	if (!interaction.isChatInputCommand()) return;

	const { commandName } = interaction;

	try {
		if (commandName === 'ping') {
			await interaction.reply('Pong!');
		} else if (commandName === 'server') {
			await interaction.reply(`Server name: ${interaction.guild.name}\nTotal members: ${interaction.guild.memberCount}`);
		} else if (commandName === 'user') {
			await interaction.reply(`Your tag: ${interaction.user.tag}\nYour id: ${interaction.user.id}`);
		} else if (commandName === 'announce') {
			const message = interaction.options.getString('message');
			await interaction.reply(`Announcement sent: ${message}`);
			interaction.channel.send(`**Announcement:** ${message}`);
		}
	} catch (error) {
		console.error(error);
		await interaction.reply({ content: 'There was an error while executing this command!', ephemeral: true });
	}
});

client.login(token);