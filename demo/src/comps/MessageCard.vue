<template>
  <div :class="message.isUser ? 'user-message' : ''">
    <p>
      <span v-text="message.sender" />:&nbsp;&nbsp;<span
        v-text="message.content"
      />
    </p>
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
div {
  display: inline-block;
  padding: 0 1rem;
  margin: 0.5rem 0;
  border-radius: 0.2rem;
  background: #ddd;

  &.user-message {
    background-color: #eee;
  }

  p > {
    margin: 0;
  }
}
</style>
