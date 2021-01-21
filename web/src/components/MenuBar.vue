<template>
  <nav class="navbar navbar-expand-lg navbar-light bg-light">
    <div class="container-fluid">
      <div>
        <label v-if="user !== null && user.servers.length > 0" class="me-2">
          <select class="form-select">
            <option
                v-for="server in user.servers"
                :key="server.id"
                :value="server.id"
            >
              {{ server.name }}
            </option>
          </select>
        </label>
        <a class="btn btn-outline-secondary"
           href="https://discord.com/oauth2/authorize?client_id=791318841384108072&scope=bot"
        >
          Add Yuri to your server
        </a>
      </div>
      <div>
        <div v-if="user !== null">
          <h3 class="me-3 align-middle">{{ user.username }} #{{ user.discriminator }}</h3>
          <a class="btn btn-danger" href="/api/auth/logout">Log out</a>
        </div>
        <div v-else>
          <a class="btn btn-primary" href="/api/auth/login">Log in</a>
        </div>
      </div>
    </div>
  </nav>
</template>

<script>
export default {
  name: "MenuBar",
  props: {
    user: {
      required: true,
      default: null,
    }
  },
}
</script>

<style scoped>
h3 {
  margin-bottom: 0;
  display: inline;
}
</style>