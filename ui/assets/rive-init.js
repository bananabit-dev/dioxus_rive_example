// Function to initialize Rive
function initRive() {
    // Check if Rive is already loaded
    if (window.rive) {
        createRiveInstance();
        return;
    }
    
    // Create a script element for Rive
    const riveScript = document.createElement('script');
    riveScript.src = 'https://unpkg.com/@rive-app/canvas@2.24.0';
    riveScript.onload = createRiveInstance;
    document.head.appendChild(riveScript);
}

// Function to create the Rive instance
function createRiveInstance() {
    // Wait for the canvas to be available
    const checkCanvas = setInterval(() => {
        const canvas = document.getElementById("rive-canvas");
        if (canvas) {
            clearInterval(checkCanvas);
            
            // Create the Rive instance
            const r = new rive.Rive({
                src: "https://cdn.rive.app/animations/vehicles.riv",
                canvas: canvas,
                autoplay: true,
                stateMachines: "bumpy",
                onLoad: () => {
                    // Ensure the drawing surface matches the canvas size and device pixel ratio
                    r.resizeDrawingSurfaceToCanvas();
                },
            });
            
            // Handle window resize to maintain proper canvas size
            window.addEventListener('resize', () => {
                r.resizeDrawingSurfaceToCanvas();
            });
        }
    }, 100);
}

// Start the initialization process
initRive();
