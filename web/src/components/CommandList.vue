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
  props: {
    guildId: {
      required: true,
      default: 0,
      type: Number,
    }
  },
  created() {
    this.fetchCommands();
  },
  methods: {
    fetchCommands() {
      fetch(`/api/commands/for_guild/${this.guildId}`)
          .then(response => response.json())
          .then(json => this.commands = json);
    }
  },
  watch: {
    guildId() {
      this.fetchCommands();
    }
  }
}
</script>

<style scoped>

</style>