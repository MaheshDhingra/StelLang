import { SlashCommandBuilder, ChatInputCommandInteraction, TextChannel, NewsChannel, ThreadChannel } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('announce')
  .setDescription('Make an announcement.')
  .addStringOption(option =>
    option.setName('message')
      .setDescription('The announcement message')
      .setRequired(true)
  );

export async function execute(interaction: ChatInputCommandInteraction) {
  const message = interaction.options.getString('message');
  await interaction.reply(`Announcement sent: ${message}`);
  if (interaction.channel && (interaction.channel.type === 0 || interaction.channel.type === 5 || interaction.channel.type === 11)) {
    await (interaction.channel as TextChannel | NewsChannel | ThreadChannel).send(`**Announcement:** ${message}`);
  }
}

module.exports = { data, execute }; 