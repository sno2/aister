<template>
  <main id="chat-page">
    <h1>Chat</h1>
    <h2>Say anything you want. The 0s and 1s won't judge you.</h2>
    <ol>
      <li
        v-for="message in messages"
        :key="`${message.sender}${message.content}`"
      >
        <MessageCard :message="message" />
      </li>
    </ol>
    <form @submit.prevent="addMessageSubmit">
      <input type="text" required placeholder="Hello there" />
      <input type="submit" value="Send" />
    </form>
  </main>
</template>

<script lang="ts">
import { defineComponent, reactive } from "vue";
import MessageCard from "../comps/MessageCard.vue";

export default defineComponent({
  components: { MessageCard },
  setup() {
    enum Sender {
      User = "You",
      Bot = "Bot",
    }

    interface Message {
      sender: Sender;
      content: string;
      isUser: boolean;
    }

    const messages = reactive<Message[]>([
      {
        sender: Sender.Bot,
        content:
          "Hello, I'm a computer-programmed figment of machine learning. Would you like to ask me something?",
        isUser: false,
      },
    ]);

    async function addMessageSubmit(e: any) {
      const $message: HTMLInputElement = e.target[0];
      messages.push({
        sender: Sender.User,
        content: $message.value,
        isUser: true,
      });
      $message.value = "";
      const res = await "I'm good, how are you?";
      setTimeout(() => {
        messages.push({
          sender: Sender.Bot,
          content: res,
          isUser: false,
        });
      }, 500);
    }

    return { messages, addMessageSubmit };
  },
});
</script>

<style lang="scss" scoped>
main#chat-page {
  padding: 0.5rem 2rem;

  > h1 {
    font-size: 5rem;
    margin: 0;
    margin-top: 1rem;
  }

  > h2 {
    font-size: 1.3rem;
    margin: 0;
    margin-bottom: 2rem;
  }

  > ol {
    padding: 0;
    list-style: none;
    max-height: 50vh;
    overflow-y: auto;
  }

  > form {
    > input {
      font-family: inherit;
    }
  }
}
</style>
