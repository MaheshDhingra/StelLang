import { SlashCommandBuilder, ChatInputCommandInteraction, PermissionFlagsBits } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('mute')
  .setDescription('Mute (timeout) a user for a specified duration.')
  .addUserOption(option =>
    option.setName('user')
      .setDescription('The user to mute')
      .setRequired(true)
  )
  .addIntegerOption(option =>
    option.setName('minutes')
      .setDescription('Duration in minutes')
      .setRequired(true)
  )
  .addStringOption(option =>
    option.setName('reason')
      .setDescription('Reason for mute')
      .setRequired(false)
  )
  .setDefaultMemberPermissions(PermissionFlagsBits.ModerateMembers);

export async function execute(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser('user');
  const minutes = interaction.options.getInteger('minutes');
  if (!user) return interaction.reply({ content: 'No user provided.', ephemeral: true });
  if (!minutes) return interaction.reply({ content: 'No duration provided.', ephemeral: true });
  const reason = interaction.options.getString('reason') || 'No reason provided';
  if (!interaction.guild) return;
  const member = await interaction.guild.members.fetch(user.id).catch(() => null);
  if (!member) return interaction.reply({ content: 'User not found in this server.', ephemeral: true });
  if (!member.moderatable) return interaction.reply({ content: 'I cannot mute this user.', ephemeral: true });
  await member.timeout(minutes * 60 * 1000, reason);
  await interaction.reply(`Muted ${user.tag} for ${minutes} minutes. Reason: ${reason}`);
}

module.exports = { data, execute }; 