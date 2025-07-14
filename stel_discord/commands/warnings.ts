import { SlashCommandBuilder, ChatInputCommandInteraction, PermissionFlagsBits } from 'discord.js';

const warnings: Record<string, string[]> = {};

export const data = new SlashCommandBuilder()
  .setName('warnings')
  .setDescription('View warnings for a user (in-memory).')
  .addUserOption(option =>
    option.setName('user')
      .setDescription('The user to view warnings for')
      .setRequired(true)
  )
  .setDefaultMemberPermissions(PermissionFlagsBits.ModerateMembers);

export async function execute(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser('user');
  if (!user) return interaction.reply({ content: 'No user provided.', ephemeral: true });
  const userWarnings = warnings[user.id] || [];
  if (userWarnings.length === 0) {
    await interaction.reply(`${user.tag} has no warnings.`);
  } else {
    await interaction.reply(`${user.tag} has ${userWarnings.length} warning(s):\n- ${userWarnings.join('\n- ')}`);
  }
}

module.exports = { data, execute }; 