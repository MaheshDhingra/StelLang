import { SlashCommandBuilder, ChatInputCommandInteraction, PermissionFlagsBits } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('ban')
  .setDescription('Ban a user from the server.')
  .addUserOption(option =>
    option.setName('user')
      .setDescription('The user to ban')
      .setRequired(true)
  )
  .addStringOption(option =>
    option.setName('reason')
      .setDescription('Reason for ban')
      .setRequired(false)
  )
  .setDefaultMemberPermissions(PermissionFlagsBits.BanMembers);

export async function execute(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser('user');
  if (!user) return interaction.reply({ content: 'No user provided.', ephemeral: true });
  const reason = interaction.options.getString('reason') || 'No reason provided';
  if (!interaction.guild) return;
  const member = await interaction.guild.members.fetch(user.id).catch(() => null);
  if (!member) return interaction.reply({ content: 'User not found in this server.', ephemeral: true });
  if (!member.bannable) return interaction.reply({ content: 'I cannot ban this user.', ephemeral: true });
  await member.ban({ reason });
  await interaction.reply(`Banned ${user.tag} for: ${reason}`);
}

module.exports = { data, execute }; 