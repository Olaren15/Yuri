<template>
  <div>
    <label>
      <select id="servers">
        <!-- will be filled when data is fetched -->
      </select>
    </label>
    <div class="container-lg">
      <CommandList v-if="user !== null"/>
    </div>
  </div>
</template>

<script>
import CommandList from "@/components/CommandList";

export default {
  name: "CommandEditor",
  components: {
    CommandList
  },
  data() {
    return {};
  },
  watch: {
    user(newUser) {
      console.log(newUser);

      if (newUser !== null && newUser.servers !== undefined) {
        let select = document.getElementById('servers');

        // clear the dropdown
        select.innerHTML = "";

        // fill with new data
        for (let i = 0; i < newUser.servers.length; i++) {
          let option = document.createElement("option");
          option.text = newUser.servers[i].name;
          option.value = newUser.servers[i].id;

          select.add(option);
        }
      }
    }
  },
  props: {
    user: {
      required: true,
      default: null,
    }
  }
}
</script>

<style scoped>

</style>