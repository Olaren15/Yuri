<template>
  <div class="card" style="width: 18rem;">
    <div class="card-body">
      <h5 class="card-title">{{ command.name }}</h5>
      <p class="card-text">{{ command.description }}</p>
      <a class="btn btn-primary" href="#">Edit</a>
    </div>
  </div>
</template>

<script>
export default {
  name: "Command",
  props: {
    command: {
      type: Object,
      required: true,
    }
  }
}
</script>

<style scoped>

</style>