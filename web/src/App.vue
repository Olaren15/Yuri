<template>
  <div id="app">
    <MenuBar v-bind:user="user"/>
  </div>
</template>

<script>
import MenuBar from "@/components/MenuBar";

export default {
  name: 'App',
  components: {
    MenuBar
  },
  data() {
    return {
      user: null,
    };
  },
  created() {
    this.fetchUser();
  },
  methods: {
    fetchUser() {
      fetch('/api/user/current')
          .then(response => response.json())
          .then(json => this.user = json);
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  height: 100vh;
}
</style>
