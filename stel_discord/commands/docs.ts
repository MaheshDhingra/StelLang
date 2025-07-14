import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';

const DOC_LINK = "https://github.com/maheshdhingra/stellang";

export const data = new SlashCommandBuilder()
  .setName('docs')
  .setDescription('Get StelLang documentation link.');

export async function execute(interaction: ChatInputCommandInteraction) {
  await interaction.reply(`StelLang documentation: ${DOC_LINK}`);
}

module.exports = { data, execute }; 