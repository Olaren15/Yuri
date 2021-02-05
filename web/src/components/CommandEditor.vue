<template>
  <div class="container-lg mt-3">
    <label>
      <select id="servers" v-model="selectedGuildId" class="form-control m-0">
        <!-- will be filled when data is fetched -->
      </select>
    </label>
    <CommandList v-if="user !== null" v-bind:guild-id="selectedGuildId"/>
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
    return {
      selectedGuildId: 0,
    };
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

          if (i === 0) {
            // automatically select first option
            select.value = newUser.servers[i].id;
          }
        }
      }
    }
  },
  props: {
    user: {
      required: true,
      default: null,
    }
  },
}
</script>

<style scoped>

</style>