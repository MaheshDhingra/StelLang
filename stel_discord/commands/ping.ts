import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('ping')
  .setDescription('Replies with pong!');

export async function execute(interaction: ChatInputCommandInteraction) {
  await interaction.reply('Pong!');
}

module.exports = { data, execute }; 