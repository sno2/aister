<template>
  <main id="translation-page">
    <h1>Translate</h1>
    <h2>Convert any of the given languages into another language!</h2>
    <form @submit.prevent="onTranslateFormSubmit">
      <table>
        <thead>
          <tr>
            <td />
            <td>Input</td>
            <td>Output</td>
          </tr>
        </thead>
        <tr>
          <td>Language</td>
          <td>
            <select v-model="state.inputLang" required>
              <option
                v-for="language in languages"
                :key="language"
                :value="language"
                v-html="language"
              />
            </select>
          </td>
          <td>
            <select v-model="state.outputLang" required>
              <option
                v-for="language in languages"
                :key="language"
                :value="language"
                v-html="language"
              />
            </select>
          </td>
        </tr>
        <tr>
          <td>Text</td>
          <td>
            <textarea v-model="state.input" required />
          </td>
          <td>
            <textarea readonly v-text="state.output" />
          </td>
        </tr>
      </table>
      <input type="submit" value="Translate" />
    </form>
  </main>
</template>

<script lang="ts">
import { defineComponent, reactive, onUnmounted } from "vue";

export default defineComponent({
  setup() {
    const enum Language {
      English,
      German,
      French,
      Spanish,
      Portugese,
      Italian,
      Catalan,
      Russian,
    }

    type LanguageStr = Lowercase<keyof typeof Language>;

    const languages: LanguageStr[] = [
      "english",
      "german",
      "french",
      "spanish",
      "portugese",
      "italian",
      "catalan",
      "russian",
    ];

    type LanguagePair = [LanguageStr, LanguageStr];

    interface TranslationData {
      pair: LanguagePair;
      input: string;
    }

    interface State {
      inputLang: LanguageStr;
      outputLang: LanguageStr;
      input: string;
      output: string;
    }
    const state = reactive<State>({
      inputLang: "english",
      outputLang: "german",
      input: "",
      output: "",
    });

    const socket = new WebSocket(
      `ws://${process.env.VUE_APP_SERVER_IP}:${process.env.VUE_APP_SERVER_PORT}/translate`
    );

    socket.onerror = () => {};

    socket.onmessage = (e) => {
      state.output = e.data;
    };

    onUnmounted(() => {
      socket.close(1000);
    });

    function onTranslateFormSubmit(e: any) {
      const { input, inputLang, outputLang } = state;
      const data: TranslationData = {
        pair: [inputLang, outputLang],
        input,
      };

      socket.send(JSON.stringify(data));
    }

    return { onTranslateFormSubmit, state, languages };
  },
});
</script>

<style lang="scss" scoped>
main#translation-page {
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
    height: 50vh;
    overflow-y: scroll;
    max-width: 45rem;
    padding-right: 1.5rem;
  }

  > form {
    td {
      padding: 0.25rem 0.5rem;
    }

    textarea,
    input,
    select {
      font-family: inherit;
    }

    select,
    textarea,
    td {
      padding: 0.2rem;
      font-size: 1rem;
    }

    tr td:first-of-type {
      text-align: right;
    }

    input[type="submit"] {
      padding: 0.2rem 0.4rem;
      font-size: 1.1rem;
      margin-top: 1rem;
    }
  }
}
</style>
