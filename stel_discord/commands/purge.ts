import { SlashCommandBuilder, ChatInputCommandInteraction, PermissionFlagsBits } from 'discord.js';

export const data = new SlashCommandBuilder()
  .setName('purge')
  .setDescription('Bulk delete messages in this channel.')
  .addIntegerOption(option =>
    option.setName('amount')
      .setDescription('Number of messages to delete (max 100)')
      .setRequired(true)
  )
  .setDefaultMemberPermissions(PermissionFlagsBits.ManageMessages);

export async function execute(interaction: ChatInputCommandInteraction) {
  const amount = interaction.options.getInteger('amount');
  if (!amount || amount < 1 || amount > 100) {
    return interaction.reply({ content: 'Please provide a number between 1 and 100.', ephemeral: true });
  }
  if (!interaction.channel || !('bulkDelete' in interaction.channel)) {
    return interaction.reply({ content: 'Cannot bulk delete in this channel.', ephemeral: true });
  }
  // @ts-ignore
  await interaction.channel.bulkDelete(amount, true);
  await interaction.reply({ content: `Deleted ${amount} messages.`, ephemeral: true });
}

module.exports = { data, execute }; 