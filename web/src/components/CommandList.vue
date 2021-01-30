<template>
  <div class="row gap-3 mt-5">
    <CommandCard v-for="command in commands" :key="command.id" class="col-12 col-md-4" v-bind:command="command"/>
  </div>
</template>

<script>
import CommandCard from "@/components/CommandCard";

export default {
  name: "CommandList",
  components: {
    CommandCard
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