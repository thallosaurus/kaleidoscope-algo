import * as z from "zod";

const { string, uuidv4, literal, discriminatedUnion, object, float32, int } = z;

export const Kaleidoscope = object({
    //texture: TextureSelection,
    texture: discriminatedUnion("type", [
        object({
            type: literal("gabor"),
            gabor_scale: float32().gte(0.0).lt(20.0),
            gabor_frequency: float32().gte(0.0).lt(20.0),
            gabor_anisotropy: float32().gte(0.0).lt(1.0),
            gabor_orientation_range: float32().gte(0.0).lt(360.0)
        }),
        object({
            type: literal("magic"),
            magic_depth: int().gte(0).lte(10),
            magic_scale: float32().gte(0.0).lte(5.0),
            magic_dist: float32().gte(0.0).lte(5.0)
        }),
        object({
            type: literal("noise"),
            noise_scale: float32().gte(1.0).lte(15.0),
            noise_detail: float32().gte(0.0).lte(5.0),
            noise_roughness: float32().gte(0.0).lte(1.0),
            noise_lacunarity: float32().gte(0.0).lte(10.0),
            noise_distortion: float32().gte(0.0).lte(10.0)
        }),
        object({
            type: literal("unoise"),
            unoise_scale: float32().gte(1.0).lte(15.0),
            unoise_detail: float32().gte(0.0).lte(5.0),
            unoise_roughness: float32().gte(0.0).lte(1.0),
            unoise_lacunarity: float32().gte(0.0).lte(10.0),
            unoise_distortion: float32().gte(0.0).lte(10.0)
        }),
        object({
            type: literal("voronoi"),
            voronoiscale: float32().gte(2.0).lte(20.0),
            voronoidetail: float32().gte(0.0).lte(3.0),
            voronoirandomize: float32().gte(0.0).lte(1.0)
        }),
        object({
            type: literal("wave"),
            wave_scale: float32().gte(0.2).lte(5.0),
            wave_distortion: float32().gte(-10.0).lte(10.0),
            wave_detail: float32().gte(0.0).lte(5.0),
            wave_detail_roughness: float32().gte(0.0).lte(1.0),
            wave_phase_offset: float32().gte(0.0).lte(50.0)
        }),
        object({
            type: literal("textured"),
            file_path: string()
        })
    ]),
    polar: object({
        repetition: int(),
        scaling: float32(),
        rotation: float32(),
        pingpong: float32(),
    }),
    composite: object({
        lens_distortion: float32().gte(-1.0).lte(-0.5),
        lens_dispersion: float32().gte(-1.0).lte(-0.5),
        hue: float32().gte(0.0).lte(1.0),
        saturation: float32().gte(1.0).lte(2.0),
    }),
    frames: discriminatedUnion("type", [
        object({
            type: literal("animated"),
            start: int().gt(0),
            end: int().gt(0)
        }),
        object({
            type: literal("still"),
            frame: int()
        })
    ]),
    id: uuidv4()
})

export type TKaleidoscope = z.infer<typeof Kaleidoscope>;