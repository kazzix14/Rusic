# frozen_string_literal: true

require_relative "jungru/version"

require "rutie"

::Rutie.new(:jungru).init "init_jungru", __dir__


  #class ::Integer
  #  def bar
  #    ::Jungru::MusicalTime.new self, 1
  #  end

  #  def th
  #    ::Jungru::MusicalTime.new 1, self
  #  end

  #  def of(other)
  #    other.numerator *= self
  #    other
  #  end
  #end
  #
  #class ::Jungru::MusicalTime
  #  attr_accessor :numerator, :denominator

  #  def initialize(numerator, denominator)
  #    raise unless numerator.is_a?(Integer) && denominator.is_a?(Integer)

  #    @numerator = numerator
  #    @denominator = denominator
  #  end

  #  def ==(other)
  #    numerator * other.denominator == other.numerator * denominator
  #  end
  #  
  #  def *(other)
  #    case other
  #    when Integer
  #      MusicalTime.new(self.numerator * other, self.denominator)
  #    when MusicalTime
  #      MusicalTime.new(self.numerator * other.numerator, self.denominator * other.denominator)
  #    else raise "not implemented MusicalTime * #{other.class}"
  #    end
  #  end
  #end

  #class ::Jungru::Track
  #  attr_accessor :name

  #  def initialize
  #    @name = nil
  #    @inst = nil
  #    @notes = {}
  #    @sections = []
  #  end
  #  
  #  def inst name
  #    @inst = ::Jungru::Instrument.new
  #    puts @inst.load 'a'
  #    puts @inst
  #    puts 'unko---------------------------------------------------'
  #    puts @inst.neko
  #    #puts @inst.value
  #  end
  #  
  #  def section
  #    @sections << ::Jungru::Section.new

  #    yield @sections.last
  #  end

  #  def symbols
  #    [*(:a..:z), *(:A..:Z), *(:'0'..:'9')].each.with_index do |k, i|
  #      @notes[k] = yield k, i
  #    end
  #  end

  #  def symbol(k, v)
  #    @notes[k] = v
  #  end
  #  
  #  def notes(section_name)
  #    s = @sections.find do |s|
  #      s.name == section_name
  #    end
  #    
  #    s.keys.map do |k|
  #      @notes[k]
  #    end
  #  end
  #  
  #  def gen_signal(composition)
  #    notes = composition.map do |section_name|
  #      self.notes(section_name)
  #    end.flatten
  #    
  #    notes.map do |note|
  #      @inst.gen_signal(note)
  #    end # TODO 時間をどうにかする
  #  end

  #  #def effect(name, *options)
  #    # TODO
  #    # @effects[name].(options)
  #  #end
  #end
  #
  #class ::Jungru::Section
  #  attr_accessor :name, :division
  #  attr_reader :keys

  #  def initialize
  #    @name = nil
  #    @division = nil
  #    @keys = []
  #  end
  #  
  #  def sheet sheet
  #    sheet.each_char do |c|
  #      case c
  #      when ' '
  #      when '.'
  #        @keys.push [nil]
  #      when /[a-zA-Z0-9]/
  #        @keys.push c.to_sym
  #      else
  #      end
  #    end
  #  end
  #end

  #class ::Jungru::Master
  #  attr_reader :composition

  #  def initialize
  #    @composition = []
  #  end
  #  
  #  def compose composition
  #    composition.each_char do |c|
  #      case c
  #      when ' '
  #      when /[a-zA-Z0-9]/
  #        @composition.push c.to_sym
  #      else
  #      end
  #    end
  #  end
  #end
  #
  #class ::Jungru::Jungru
  #  attr_reader :master, :tracks

  #  def initialize
  #    @master = ::Jungru::Master.new
  #    @tracks = []
  #  end
  #  
  #  def master
  #    yield @master
  #  end

  #  def track
  #    @tracks << ::Jungru::Track.new

  #    yield @tracks.last
  #  end
  #  
  #  def play()
  #    signals = @tracks.map do |t|
  #      t.gen_signal(@master.composition)
  #    end
  #    
  #    # TODO
  #    # signalsをミックスする
  #  end
  #end
#end
