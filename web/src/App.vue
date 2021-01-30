<template>
  <div id="app">
    <MenuBar v-bind:user="user"/>
    <CommandEditor v-bind:user="user"/>
  </div>
</template>

<script>
import MenuBar from "@/components/MenuBar";
import CommandEditor from "@/components/CommandEditor";

export default {
  name: 'App',
  components: {
    MenuBar,
    CommandEditor
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
          .then(user => {
            if (user !== null) {
              this.fetchServers().then(servers => {
                user.servers = servers;
                this.user = user;
              });
            } else {
              this.user = user;
            }
          });
    },
    fetchServers() {
      return fetch('/api/servers/in_common')
          .then(response => response.json());
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
