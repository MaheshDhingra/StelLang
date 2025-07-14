import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('user')
  .setDescription('Displays user information.');

export async function execute(interaction: ChatInputCommandInteraction) {
  await interaction.reply(`Your tag: ${interaction.user.tag}\nYour id: ${interaction.user.id}`);
}

module.exports = { data, execute }; 