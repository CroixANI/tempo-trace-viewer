<script lang="ts">
  import TagsSection from './TagsSection.svelte';
  import type { KeyValue } from '../types/trace';

  export let attributes: KeyValue[];
</script>

<TagsSection {attributes} title="Resource / Process" />
