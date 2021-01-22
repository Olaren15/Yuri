<template>
  <div class="row gap-3 mt-5">
    <Command v-for="command in commands" :key="command.id" class="col-12 col-md-4" v-bind:command="command"/>
  </div>
</template>

<script>
import Command from "@/components/Command";

export default {
  name: "CommandList",
  components: {
    Command
  },
  data() {
    return {
      commands: [],
    }
  },
  created() {
    this.fetchCommands();
  },
  methods: {
    fetchCommands() {
      fetch('/api/commands/all')
          .then(response => response.json())
          .then(json => this.commands = json);
    }
  }
}
</script>

<style scoped>

</style>