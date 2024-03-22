<script lang="ts">
    import * as SimpleIcons from 'simple-icons';
    import * as MdiIcons from '@mdi/js';

    export let svg: string;
    export let alt: string;
    export let fill = '#fafafa';
    export let classes = '';

    let path: string = '';

    if (svg in SimpleIcons) {
        path = SimpleIcons[svg].path;
    } else if (svg in MdiIcons) {
        path = MdiIcons[svg];
    } else {
        path = svg;
    }
</script>

<div class="{classes}">
    <svg class="w-9/10" {fill} viewBox="0 0 24 24">
        <title>{alt}</title>
        <path d={path} />
    </svg>
</div>
