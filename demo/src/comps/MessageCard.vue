<template>
  <div :class="`${message.isUser ? 'user-message' : ''} message`">
    <div>
      <p>
        <span v-text="message.content" />
      </p>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  props: {
    message: {
      required: true,
    },
  },
  setup(props) {
    return props;
  },
});
</script>

<style lang='scss' scoped>
.message {
  display: flex;
  width: 100%;
  align-items: space-between;

  &.user-message {
    margin-left: auto;

    > div {
      background-color: #eee;
      margin-left: 0;
    }
  }

  &.bot-message {
    margin-left: auto;
  }

  > div {
    display: inline-block;
    padding: 0 1rem;
    margin: 0.75rem 0;
    border-radius: 0.2rem;
    background: #ddd;
    margin-left: auto;
    max-width: 85%;

    p > {
      margin: 0;
    }
  }
}
</style>
