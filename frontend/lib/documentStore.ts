"use client";

type Listener = () => void;

interface DocumentState {
    title: string;
    description: string;
    isPublic: boolean;
    uniforms: {
        seed: number;
        subtraction: number;
    };
}

const initialState: DocumentState = {
    title: "Ray Marching Test",
    description: "",
    isPublic: false,
    uniforms: {
        seed: 0.5,
        subtraction: 0,
    },
};

class DocumentStore {
    private state: DocumentState = { ...initialState };
    private listeners = new Set<Listener>();

    getState(): DocumentState {
        return this.state;
    }

    setState(partial: Partial<DocumentState>): void {
        this.state = { ...this.state, ...partial };
        this.notify();
    }

    setUniform(key: keyof DocumentState["uniforms"], value: number): void {
        this.state = {
            ...this.state,
            uniforms: { ...this.state.uniforms, [key]: value },
        };
        this.notify();
    }

    subscribe(listener: Listener): () => void {
        this.listeners.add(listener);
        return () => this.listeners.delete(listener);
    }

    private notify(): void {
        this.listeners.forEach((listener) => listener());
    }
}

export const documentStore = new DocumentStore();
