import { SlashCommandBuilder, ChatInputCommandInteraction, PermissionFlagsBits } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('unmute')
  .setDescription('Unmute (remove timeout) from a user.')
  .addUserOption(option =>
    option.setName('user')
      .setDescription('The user to unmute')
      .setRequired(true)
  )
  .setDefaultMemberPermissions(PermissionFlagsBits.ModerateMembers);

export async function execute(interaction: ChatInputCommandInteraction) {
  const user = interaction.options.getUser('user');
  if (!user) return interaction.reply({ content: 'No user provided.', ephemeral: true });
  if (!interaction.guild) return;
  const member = await interaction.guild.members.fetch(user.id).catch(() => null);
  if (!member) return interaction.reply({ content: 'User not found in this server.', ephemeral: true });
  if (!member.moderatable) return interaction.reply({ content: 'I cannot unmute this user.', ephemeral: true });
  await member.timeout(null);
  await interaction.reply(`Unmuted ${user.tag}.`);
}

module.exports = { data, execute }; 