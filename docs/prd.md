# Piano Sight Reading Application - Product Requirements Document

## Overview

A desktop music sight reading application built exclusively in Rust that provides interactive piano practice through MIDI keyboard integration and real-time visual feedback.

## Core Features

### 1. Musical Notation Rendering
- **Clef Support**: Render both treble and bass clefs with proper note positioning
- **Staff Display**: Standard 5-line musical staff for each clef
- **Note Visualization**: Display quarter notes, half notes, whole notes, and eighth notes with correct positioning
- **Key Signatures**: Support for major and minor key signatures
- **Time Signatures**: Support for common time signatures (4/4, 3/4, 2/4)

### 2. MIDI Integration
- **Input Capture**: Real-time MIDI input from connected piano/keyboard devices
- **Note Detection**: Accurately capture pressed keys with velocity and timing information
- **Multi-note Support**: Handle simultaneous note presses for chords and intervals
- **Device Management**: Auto-detect and connect to available MIDI devices

### 3. Interactive Learning System
- **Note-by-Note Practice**: Allow users to progress through songs at their own pace
- **Simultaneous Note Support**: Handle chord progressions and multiple notes as required by the musical piece
- **Real-time Feedback**: Immediate visual response to user input
- **Progress Tracking**: Monitor user advancement through songs and difficulty levels

### 4. Visual Feedback System
- **Correct Notes**: Highlight successfully played notes in green
- **Incorrect Notes**: Mark wrong or missed notes in red
- **Progress Indication**: Visual progress bar or indicator showing completion status
- **Dynamic Scrolling**: Automatically advance the musical score as the user progresses

### 5. Music Library Management
- **Free MIDI Songs**: Curated collection of royalty-free MIDI compositions
- **Difficulty Levels**: Songs categorized by complexity (Beginner, Intermediate, Advanced)
- **Level Progression**: Structured learning path from simple melodies to complex pieces
- **Song Selection**: User-friendly interface for browsing and selecting practice pieces

## Technical Requirements

### Platform
- **Language**: Rust exclusively
- **Target**: Desktop application (cross-platform compatibility)
- **Architecture**: Native application with efficient real-time audio processing

### Performance
- **Latency**: Sub-20ms response time for MIDI input processing
- **Rendering**: Smooth 60fps visual updates
- **Memory**: Efficient memory usage for extended practice sessions

### Dependencies (Rust Crates)
- MIDI processing and device integration
- 2D graphics rendering for musical notation
- Audio/MIDI file parsing
- Cross-platform windowing and UI framework

## User Experience

### Workflow
1. **Setup**: Connect MIDI keyboard and launch application
2. **Song Selection**: Choose practice piece based on skill level
3. **Practice Mode**: Play notes as they appear on the staff
4. **Feedback**: Receive immediate visual confirmation of correct/incorrect notes
5. **Progress**: Advance through the piece with automatic scrolling
6. **Completion**: Review performance and select next piece

### Learning Progression
- **Level 1**: Single notes, simple melodies, treble clef only
- **Level 2**: Basic bass clef, simple two-hand coordination
- **Level 3**: Chord progressions, complex rhythms
- **Level 4**: Advanced pieces with full keyboard utilization

## Success Criteria
- Accurate MIDI input detection with minimal latency
- Clear, readable musical notation rendering
- Intuitive user interface requiring minimal learning curve
- Progressive difficulty system that builds skills systematically
- Stable performance during extended practice sessions

## Future Enhancements (Post-MVP)
- Metronome integration
- Recording and playback functionality
- Custom song import capabilities
- Performance analytics and progress tracking
- Multiple instrument support beyond piano