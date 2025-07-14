import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';

const COMMON_ERRORS: Record<string, string> = {
  "syntax": "Check for missing semicolons, brackets, or typos in your code.",
  "module not found": "Make sure the module is installed and the import path is correct.",
  "permission": "Check your permissions and try running as admin/root if needed."
};

export const data = new SlashCommandBuilder()
  .setName('helpme')
  .setDescription('Get help with common errors.')
  .addStringOption(option =>
    option.setName('error')
      .setDescription('Describe your error (e.g., syntax, module not found, permission)')
      .setRequired(true)
  );

export async function execute(interaction: ChatInputCommandInteraction) {
  const error = interaction.options.getString('error')?.toLowerCase() || '';
  const help = COMMON_ERRORS[error] || 'Sorry, I do not have info on that error yet.';
  await interaction.reply(`Help for **${error}**: ${help}`);
}

module.exports = { data, execute }; 