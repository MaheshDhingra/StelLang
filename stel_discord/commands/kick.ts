import { SlashCommandBuilder, ChatInputCommandInteraction, PermissionFlagsBits } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('kick')
  .setDescription('Kick a user from the server.')
  .addUserOption(option =>
    option.setName('user')
      .setDescription('The user to kick')
      .setRequired(true)
  )
  .addStringOption(option =>
    option.setName('reason')
      .setDescription('Reason for kick')
      .setRequired(false)
  )
  .setDefaultMemberPermissions(PermissionFlagsBits.KickMembers);

export async function execute(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser('user');
  if (!user) return interaction.reply({ content: 'No user provided.', ephemeral: true });
  const reason = interaction.options.getString('reason') || 'No reason provided';
  if (!interaction.guild) return;
  const member = await interaction.guild.members.fetch(user.id).catch(() => null);
  if (!member) return interaction.reply({ content: 'User not found in this server.', ephemeral: true });
  if (!member.kickable) return interaction.reply({ content: 'I cannot kick this user.', ephemeral: true });
  await member.kick(reason);
  await interaction.reply(`Kicked ${user.tag} for: ${reason}`);
}

module.exports = { data, execute }; 