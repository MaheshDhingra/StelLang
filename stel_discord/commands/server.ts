import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('server')
  .setDescription('Displays server information.');

export async function execute(interaction: ChatInputCommandInteraction) {
  await interaction.reply(`Server name: ${interaction.guild?.name}\nTotal members: ${interaction.guild?.memberCount}`);
}

module.exports = { data, execute }; 