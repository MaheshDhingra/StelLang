import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';
import fetch from 'node-fetch';

export const data = new SlashCommandBuilder()
  .setName('askai')
  .setDescription('Ask Hack Club AI anything!')
  .addStringOption(option =>
    option.setName('question')
      .setDescription('Your question for the AI')
      .setRequired(true)
  );

export async function execute(interaction: ChatInputCommandInteraction) {
  const question = interaction.options.getString('question');
  await interaction.deferReply();
  try {
    const aiRes = await fetch('https://ai.hackclub.com/chat/completions', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        messages: [{ role: 'user', content: question }]
      })
    });
    const aiData = await aiRes.json();
    const aiMsg = aiData.choices?.[0]?.message?.content || 'AI did not return a response.';
    await interaction.editReply(aiMsg);
  } catch (err) {
    await interaction.editReply('Error contacting Hack Club AI.');
  }
}

module.exports = { data, execute }; 