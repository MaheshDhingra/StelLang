import { SlashCommandBuilder, ChatInputCommandInteraction, PermissionFlagsBits } from 'discord.js';

const warnings: Record<string, string[]> = {};

export const data = new SlashCommandBuilder()
  .setName('warn')
  .setDescription('Warn a user (stored in memory).')
  .addUserOption(option =>
    option.setName('user')
      .setDescription('The user to warn')
      .setRequired(true)
  )
  .addStringOption(option =>
    option.setName('reason')
      .setDescription('Reason for warning')
      .setRequired(false)
  )
  .setDefaultMemberPermissions(PermissionFlagsBits.ModerateMembers);

export async function execute(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser('user');
  if (!user) return interaction.reply({ content: 'No user provided.', ephemeral: true });
  const reason = interaction.options.getString('reason') || 'No reason provided';
  if (!warnings[user.id]) warnings[user.id] = [];
  warnings[user.id].push(reason);
  await interaction.reply(`Warned ${user.tag}. Reason: ${reason}`);
}

module.exports = { data, execute }; 