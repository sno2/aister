<template>
  <header>
    <h3><RouterLink to="/">Aister</RouterLink></h3>
    <nav>
      <RouterLink to="/chat">Chat</RouterLink>
      <RouterLink to="/translate">Translation</RouterLink>
      <RouterLink to="/translate">Entity Determination</RouterLink>
    </nav>
    <a href="https://github.com/sno2/aister" target="_blank">View Code</a>
  </header>
</template>

<style lang="scss" scoped>
header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
  padding: 0.5rem 2rem;
  border-bottom: adjust-color(#000, $alpha: -0.7) dotted 1px;

  > h3 > a {
    text-decoration: none;
    color: inherit;
    font-size: 1.75rem;
  }

  > nav {
    display: flex;

    a {
      padding: 0.5rem 0.6rem;
      color: inherit;
      text-decoration: none;
      font-size: 1.1rem;
      $nav-links-color: #000;
      color: adjust-color($nav-links-color, $alpha: -0.25);

      &:hover {
        color: $nav-links-color;
      }
    }
  }

  > a {
    padding: 0.75rem 1.3rem;
    border-radius: 5rem;
    font-size: 1.15rem;
    color: inherit;
    text-decoration: none;
    $nav-cta-border: black;
    border: 1px solid adjust-color($nav-cta-border, $alpha: -0.8);

    &:hover {
      background-color: $nav-cta-border;
      color: white;
      border: 1px solid adjust-color($nav-cta-border, $alpha: -0.2);
    }
  }
}
</style>
