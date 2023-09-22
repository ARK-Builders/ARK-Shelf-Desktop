export interface SystemTime {
    secs_since_epoch: number;
    nanos_since_epoch: number;
}

export interface LinkInfo {
    title: string;
    desc?: string;
    url: string;
    name: string;
    created_time?: SystemTime;
    score?: LinkScoreMap;
    graph?: GraphInfo;
}
export interface OpenGraph {
    /// Represents the "og:title" OpenGraph meta tag.
    ///
    /// The title of your object as it should appear within
    /// the graph, e.g., "The Rock".
    title?: string;
    /// Represents the "og:description" OpenGraph meta tag
    description?: string;
    /// Represents the "og:url" OpenGraph meta tag
    url?: string;
    /// Represents the "og:image" OpenGraph meta tag
    image?: string;
    /// Represents the "og:type" OpenGraph meta tag
    ///
    /// The type of your object, e.g., "video.movie". Depending on the type
    /// you specify, other properties may also be required.
    object_type?: string;
    /// Represents the "og:locale" OpenGraph meta tag
    locale?: string;
}
export interface LinkScoreMap {
    name: string;
    value: number;
    id: string;
}

export type SortMode = 'normal' | 'date' | 'score';

export type PreviewLoaded = {
    url: string;
    graph: GraphInfo;
    createdTime: SystemTime;
};

export type GraphInfo = {
    imageUrl?: string;
    title?: string;
    description?: string;
};
