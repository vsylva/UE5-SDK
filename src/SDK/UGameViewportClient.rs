#[struct_macro::inherit(super::UScriptViewportClient::UScriptViewportClient, Flags)]
#[derive(Debug, Clone, Copy,)]
#[repr(C)]
pub struct UGameViewportClient {
    #[offset(0x00B0)]
    pub ViewModelIndex: u32,

    #[offset(0x00B8)]
    pub ShowFlags: FEngineShowFlags,
}

#[struct_macro::bitfields]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default,)]
pub struct FEngineShowFlags {
    #[bits(8, word_0)]
    pub PostProcessing: bool,
    pub Bloom:          bool,
    pub LocalExposure:  bool,
    pub Tonemapper:     bool,
    pub AntiAliasing:   bool,
    pub TemporalAA:     bool,
    pub AmbientCubemap: bool,
    pub EyeAdaptation:  bool,

    #[bits(8, word_1)]
    pub VisualizeHDR: bool,
    pub VisualizeSkyLightIlluminance: bool,
    pub VisualizeLocalExposure: bool,
    pub LensFlares: bool,
    pub LensDistortion: bool,
    pub GlobalIllumination: bool,
    pub Vignette: bool,
    pub Grain: bool,

    #[bits(8, word_2)]
    pub AmbientOcclusion:           bool,
    pub Decals:                     bool,
    pub CameraImperfections:        bool,
    pub OnScreenDebug:              bool,
    pub OverrideDiffuseAndSpecular: bool,
    pub LightingOnlyOverride:       bool,
    pub ReflectionOverride:         bool,
    pub VisualizeBuffer:            bool,

    #[bits(8, word_3)]
    pub VisualizeNanite:           bool,
    pub VisualizeLumen:            bool,
    pub VisualizeSubstrate:        bool,
    pub VisualizeGroom:            bool,
    pub VisualizeVirtualShadowMap: bool,
    pub DirectLighting:            bool,
    pub DirectionalLights:         bool,
    pub PointLights:               bool,

    #[bits(8, word_4)]
    pub SpotLights:                bool,
    pub RectLights:                bool,
    pub ColorGrading:              bool,
    pub VectorFields:              bool,
    pub DepthOfField:              bool,
    pub GBufferHints:              bool,
    pub MotionBlur:                bool,
    pub CompositeEditorPrimitives: bool,

    #[bits(8, word_5)]
    pub OpaqueCompositeEditorPrimitives: bool,
    pub TestImage: bool,
    pub VisualizeDOF: bool,
    pub VertexColors: bool,
    pub PhysicalMaterialMasks: bool,
    pub Refraction: bool,
    pub CameraInterpolation: bool,
    pub SceneColorFringe: bool,

    #[bits(8, word_6)]
    pub ToneCurve:                 bool,
    pub SeparateTranslucency:      bool,
    pub ScreenPercentage:          bool,
    pub VisualizeMotionBlur:       bool,
    pub VisualizeMotionVectors:    bool,
    pub VisualizeReprojection:     bool,
    pub VisualizeTemporalUpscaler: bool,
    pub VisualizeTSR:              bool,

    #[bits(8, word_7)]
    pub MegaLights:                 bool,
    pub ReflectionEnvironment:      bool,
    pub VisualizeOutOfBoundsPixels: bool,
    pub Diffuse:                    bool,
    pub Specular:                   bool,
    pub SelectionOutline:           bool,
    pub ScreenSpaceReflections:     bool,
    pub LumenReflections:           bool,

    #[bits(8, word_8)]
    pub ContactShadows: bool,
    pub RayTracedDistanceFieldShadows: bool,
    pub CapsuleShadows: bool,
    pub SubsurfaceScattering: bool,
    pub VisualizeSSS: bool,
    pub VolumetricLightmap: bool,
    pub IndirectLightingCache: bool,
    pub DebugAI: bool,

    #[bits(8, word_9)]
    pub VisLog:                bool,
    pub Navigation:            bool,
    pub GameplayDebug:         bool,
    pub TexturedLightProfiles: bool,
    pub LightFunctions:        bool,
    pub InstancedStaticMeshes: bool,
    pub InstancedFoliage:      bool,
    pub HISMCOcclusionBounds:  bool,

    #[bits(8, word_10)]
    pub HISMCClusterTree:         bool,
    pub VisualizeInstanceUpdates: bool,
    pub InstancedGrass:           bool,
    pub DynamicShadows:           bool,
    pub Particles:                bool,
    pub Niagara:                  bool,
    pub HeterogeneousVolumes:     bool,
    pub SkeletalMeshes:           bool,

    #[bits(8, word_11)]
    pub BuilderBrush:           bool,
    pub Translucency:           bool,
    pub BillboardSprites:       bool,
    pub LOD:                    bool,
    pub LightComplexity:        bool,
    pub ShaderComplexity:       bool,
    pub StationaryLightOverlap: bool,
    pub LightMapDensity:        bool,

    #[bits(8, word_12)]
    pub StreamingBounds:     bool,
    pub Constraints:         bool,
    pub MassProperties:      bool,
    pub CameraFrustums:      bool,
    pub AudioRadius:         bool,
    pub ForceFeedbackRadius: bool,
    pub BSPSplit:            bool,
    pub Brushes:             bool,

    #[bits(8, word_13)]
    pub Lighting:         bool,
    pub DeferredLighting: bool,
    pub Editor:           bool,
    pub BSPTriangles:     bool,
    pub LargeVertices:    bool,
    pub Grid:             bool,
    pub Snap:             bool,
    pub MeshEdges:        bool,

    #[bits(8, word_14)]
    pub Cover: bool,
    pub Splines: bool,
    pub Selection: bool,
    pub VisualizeLevelInstanceEditing: bool,
    pub EditingLevelInstance: bool,
    pub ModeWidgets: bool,
    pub Bounds: bool,
    pub HitProxies: bool,

    #[bits(8, word_15)]
    pub LightInfluences: bool,
    pub Pivot:           bool,
    pub ShadowFrustums:  bool,
    pub Wireframe:       bool,
    pub Materials:       bool,
    pub StaticMeshes:    bool,
    pub Landscape:       bool,
    pub LightRadius:     bool,

    #[bits(8, word_16)]
    pub Fog:                 bool,
    pub Volumes:             bool,
    pub Game:                bool,
    pub ActorColoration:     bool,
    pub BSP:                 bool,
    pub Collision:           bool,
    pub CollisionVisibility: bool,
    pub CollisionPawn:       bool,

    #[bits(8, word_17)]
    pub LightShafts:           bool,
    pub PostProcessMaterial:   bool,
    pub Atmosphere:            bool,
    pub Cloud:                 bool,
    pub CameraAspectRatioBars: bool,
    pub CameraSafeFrames:      bool,
    pub TextRender:            bool,
    pub Rendering:             bool,

    #[bits(8, word_18)]
    pub HighResScreenshotMask:    bool,
    pub HMDDistortion:            bool,
    pub StereoRendering:          bool,
    pub DistanceCulledPrimitives: bool,
    pub VisualizeLightCulling:    bool,
    pub PrecomputedVisibility:    bool,
    pub SkyLighting:              bool,
    pub PreviewShadowsIndicator:  bool,

    #[bits(8, word_19)]
    pub PrecomputedVisibilityCells:   bool,
    pub VisualizeVolumetricLightmap:  bool,
    pub VolumeLightingSamples:        bool,
    pub Paper2DSprites:               bool,
    pub VisualizeDistanceFieldAO:     bool,
    pub VisualizeMeshDistanceFields:  bool,
    pub PhysicsField:                 bool,
    pub VisualizeGlobalDistanceField: bool,

    #[bits(8, word_20)]
    pub VisualizeLightingOnProbes: bool,
    pub ScreenSpaceAO:             bool,
    pub DistanceFieldAO:           bool,
    pub LumenGlobalIllumination:   bool,
    pub VolumetricFog:             bool,
    pub VisualizeSSR:              bool,
    pub VisualizeShadingModels:    bool,
    pub VisualizeSenses:           bool,

    #[bits(8, word_21)]
    pub LODColoration:                    bool,
    pub HLODColoration:                   bool,
    pub QuadOverdraw:                     bool,
    pub ShaderComplexityWithQuadOverdraw: bool,
    pub PrimitiveDistanceAccuracy:        bool,
    pub MeshUVDensityAccuracy:            bool,
    pub MaterialTextureScaleAccuracy:     bool,
    pub OutputMaterialTextureScales:      bool,

    #[bits(8, word_22)]
    pub RequiredTextureResolution: bool,
    pub VirtualTexturePendingMips: bool,
    pub WidgetComponents:          bool,
    pub Bones:                     bool,
    pub ServerDrawDebug:           bool,
    pub MediaPlanes:               bool,
    pub VREditing:                 bool,
    pub OcclusionMeshes:           bool,

    #[bits(8, word_23)]
    pub VisualizeInstanceOcclusionQueries: bool,
    pub DisableOcclusionQueries: bool,
    pub PathTracing: bool,
    pub RayTracingDebug: bool,
    pub VisualizeSkyAtmosphere: bool,
    pub VisualizeLightFunctionAtlas: bool,
    pub VisualizeCalibrationColor: bool,
    pub VisualizeCalibrationGrayscale: bool,

    #[bits(8, word_24)]
    pub VisualizeCalibrationCustom:                  bool,
    pub VisualizePostProcessStack:                   bool,
    pub VirtualTexturePrimitives:                    bool,
    pub VisualizeVolumetricCloudConservativeDensity: bool,
    pub VisualizeVolumetricCloudEmptySpaceSkipping:  bool,
    pub VirtualShadowMapPersistentData:              bool,
    pub DebugDrawDistantVirtualSMLights:             bool,
    pub VirtualTextureResidency:                     bool,

    #[bits(8, word_25)]
    pub InputDebugVisualizer:            bool,
    pub LumenScreenTraces:               bool,
    pub LumenDetailTraces:               bool,
    pub LumenGlobalTraces:               bool,
    pub LumenFarFieldTraces:             bool,
    pub LumenSecondaryBounces:           bool,
    pub LumenShortRangeAmbientOcclusion: bool,
    pub NaniteMeshes:                    bool,

    #[bits(8, word_26)]
    pub NaniteStreamingGeometry: bool,
    pub VisualizeGPUSkinCache: bool,
    pub VisualizeLWCComplexity: bool,
    pub ShaderPrint: bool,
    pub SceneCaptureCopySceneDepth: bool,
    pub Cameras: bool,
    pub Hair: bool,
    _pad_215: bool,
}
