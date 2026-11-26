export type CollectionType =
  | 'overview'
  | 'palettes'
  | 'typography'
  | 'components'
  | 'icons'
  | 'effects'
  | 'animations'
  | 'layouts'
  | 'compare'
  | 'export'
  | 'sources';

export interface License {
  status: 'reusable' | 'reference-only' | 'unknown';
  basis: string;
  spdx?: string;
}

export interface Provenance {
  source: string;
  url: string;
  selector?: string;
  viewport: string;
  scheme: string;
  captured_at: string;
}

export interface Item<T = any> {
  $schema?: string;
  id: string;
  item_type: string;
  name?: string;
  slug: string;
  description?: string;
  tags: string[];
  data: T;
  provenance: Provenance[];
  license: License;
  confidence: number;
  similar_to?: string[];
  created_at: string;
  updated_at: string;
}

// Palette Data
export interface ContrastPair {
  fg_hex: string;
  bg_hex: string;
  ratio: number;
  wcag_aa: boolean;
  wcag_aaa: boolean;
}

export interface PaletteColor {
  hex: string;
  role: string;
  usage_weight: number;
  oklch: [number, number, number];
  contrast_pairs: ContrastPair[];
}

export interface PaletteData {
  scheme: 'light' | 'dark' | 'adaptive';
  colors: PaletteColor[];
}

// Typography Data
export interface FontFamily {
  name: string;
  stack: string[];
  weights: number[];
  is_custom: boolean;
}

export interface TypeScaleEntry {
  role: string;
  size_px: number;
  line_height: number;
  weight: number;
  letter_spacing?: string;
}

export interface TypographyData {
  families: FontFamily[];
  scale: TypeScaleEntry[];
}

// Component Data
export interface StateStyles {
  background?: string;
  color?: string;
  border?: string;
  shadow?: string;
  opacity?: number;
  transform?: string;
}

export interface ComponentVariant {
  role: 'button' | 'input' | 'card' | 'badge';
  variant: string;
  states: {
    default: StateStyles;
    hover?: StateStyles;
    focus?: StateStyles;
    active?: StateStyles;
    disabled?: StateStyles;
  };
}

export interface ComponentData {
  role: string;
  skeleton_html: string;
  variants: ComponentVariant[];
}

// Icon Data
export interface IconData {
  name: string;
  svg_content: string;
  view_box: string;
  geometry_hash: string;
  has_current_color: boolean;
  matched_set?: string;
}

// Effect Data
export interface EffectData {
  kind: 'shadow' | 'glassmorphism' | 'gradient' | 'blur';
  css_value: string;
  preview_bg?: string;
}

// Animation Data
export interface Keyframe {
  offset: number;
  styles: Record<string, string>;
}

export interface Timing {
  duration_ms: number;
  easing: string;
  iterations: number;
}

export interface AnimationData {
  name: string;
  driver: 'css-keyframe' | 'css-transition' | 'waapi';
  keyframes: Keyframe[];
  timing: Timing;
}

// Layout Data
export interface LayoutData {
  section_name: string;
  display: 'flex' | 'grid' | 'block';
  columns?: string;
  gap?: string;
  wireframe_svg?: string;
}
